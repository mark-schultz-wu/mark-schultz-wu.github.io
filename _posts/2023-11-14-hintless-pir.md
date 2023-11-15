---
layout: post
title: Hintless Single-Server Private Information Retrieval
---

This summer, I was hosted by Mariana Raykova and Baiyu Li at Google to work on Single Server PIR.
This internship led to a [paper](https://eprint.iacr.org/2023/1733).
Rather than try to put together some tweet-thread (or X-thread? who knows) talking about the result,
I thought it best to write it up somewhere that supports latex, e.g. here.

# Private Information Retrieval

Private Information Retrieval is the problem of "securely querying" a database.
In particular, you have a client, and a server, who holds a *public* database.
The client wants to retrieve a particular record from this database without the server learning which record was retrieved.
Note that there is an obvious trivial protocol --- the server transmits the whole database!
The client can then locally retrieve the record they want, and the server learns nothing.

This scheme is actually fairly (computationally) efficient.
The server has a linear scan through the database, and the client can stream the database and only store their desired record,
e.g. the memory requirements on the client are fairily small.
The bandwidth of the scheme is very large though, and the main goal in PIR research is to reduce these bandwidth requirements.

This is done various ways. Typically, authors like to add some "extra tools" to the standard PIR setting, for example

1. [SimplePIR](https://eprint.iacr.org/2022/949): Having a (query-independent) "hint" the client downloads beforehand, that they can amortize over all queries.
2. [TreePIR](https://eprint.iacr.org/2023/204): Assuming the database is held by two non-colluding servers.
3. [Piano](https://eprint.iacr.org/2023/452): Assume the entire database is streamed to a memory-bound client beforehand.

Under these assumptions, one can get practically efficient schemes, where efficiency metrics tend to be
* total bandwidth cost, and
* server computational cost.

For example, SimplePIR has throughput comperable to the memory bandwidth of certain processors,
or in other words server's computational cost in SimplePIR is comperable to a linear scan through the database.
And Piano is even faster!

There do exist schemes that avoid these assumptions (for example [SpiralPIR](https://eprint.iacr.org/2022/368)) which are still pretty fast, but
there's a pretty noticable slowdown --- in our measurements SpiralPIR was at least an order of magnitude slower than SimplePIR.
Our goal was to bridge this gap --- give a scheme that avoided the above three assumptions, but had better server computation than SpiralPIR.
We ended up with a scheme that achieves this goal "asymptotically", meaning the cost to execute it is (in the limit of large database sizes) identical
to that of SimplePIR.
We measured that this leads to concretely small overhead (for certain "large enough" databases of practically relevant sizes), e.g. we can remove SimplePIR's hint at low cost.

In the interest of brevity, I'll focus on describing the main idea behind our main construction, as well as a particular optimization that we introduced that I like quite a bit (and is theoretically an order-of-magnitude speedup, though practically I think we measured it as a 6x speedup).

# Single Server PIR from LWE

We start with the SimplePIR (and FrodoPIR) construction.
This leverages the following simple idea.
If you format an $$m$$-record database $$\mathsf{DB}$$ as a matrix $$\mathsf{DB}\in \mathbb{Z}_p^{\sqrt{m}\times \sqrt{m}}$$,
then one can retrieve the $$i$$th column of this database by multiplying it via the $$i$$th basis vector $$e_i$$.
This can then be computed using homomorphic encryption, namely

$$\mathsf{DB}, \mathsf{Enc}(e_i)\mapsto \mathsf{Enc}(\mathsf{DB}\cdot e_i).$$

All one needs is an encryption scheme that supports efficient homomorphic matrix-vector multiplication.
Learning with Errors (LWE)-based encryption does just this!
I've written some [blog posts]({% post_url 2022-07-05-nist-standard-out %}) on this in the past geared towards a general audience,
so I will be brief below.
LWE-based ciphertexts take the form

$$C := [A, A\cdot s + f + (q/p)m]\in\mathbb{Z}_q^{\sqrt{m}\times n} \times \mathbb{Z}_q^{\sqrt{m}\times 1}$$

where $$A$$ is a uniformly-random matrix, $$s$$ is a (typically short) secret vector, $$f$$ is a (short and secret) "noise" vector, and $$m\in\mathbb{Z}_p^{\sqrt{m}}$$ is the message one is encrypting.
One can efficient compute matrix-vector products using this, as

$$\mathsf{DB}\cdot C := [\mathsf{DB}\cdot A, \mathsf{DB}\cdot A\cdot s + \mathsf{DB}\cdot f + (q/p)\mathsf{DB}\cdot m]$$

There are some (routine) details that one has to get right with the above scheme, but for the most part the fact that it yields a *correct* homomorphic scheme is standard.
What is harder is getting it to be *bandwidth-efficient*.

The issue is in the transmission of $$A\in\mathbb{Z}_q^{\sqrt{m}\times n}$$, and in particular its dependence on $$n$$.
This is the lattice dimension, and is required to be $$\geq 500$$ for security purposes (though depending on how large $$q$$ is this can grow much higher, e.g. in the thousands).
This means that the "$$A$$" component of ciphertexts is more than $$500\times$$ larger than the component containing the message.
There is a standard trick to "compress" $$A$$ to a PRG [^prg] seed $$sd$$, which is then expanded to $$A := \mathsf{PRG}(sd)$$.
This works to minimize the size of the client's encryption.
The issue is that when the server computes

$$\mathsf{DB}\cdot A = \mathsf{DB}\cdot \mathsf{PRG}(sd)$$

[^prg]: Technically one needs this to be a hashing-based primitive to be able to get provable security out of it. The term to search is "Extendible output function", but I won't belabor the point.

there is no way to compress this back down to a similarly short seed $$sd'$$.
The typical way to circumvent this issue is to using something like Ring Learning with Errors-based schemes, where the "$$A$$" component
is the same size as the message-dependent component.
This has two issues in our setting though.

1. Our homomorphic computation is a matrix-vector multiplication. This is quite efficient for LWE, but less so for RLWE.
2. For LWE-based encryption, one can freely choose the modulus $$q$$. For example, one can set $$q \in \{2^{32}, 2^{64}\}$$ for fast hardware-based arithmetic.
   For RLWE-based encryption, there are often restrictions on $$q$$, and $$q\in\{2^{32}, 2^{64}\}$$ didn't work for our protocols.

In our setting we are able to (almost) completely solve the first problem.
In particular, we give an RLWE-based protocol to securely compute matrix-vector multiplication that is practically (when measured in $$\mathbb{Z}_q$$ operations) only a small constant factor larger than the corresponding plaintext computation.
This amounts to *heavily* optimizing a standard protocol (the ["Diagonally Dominant" multiplication algoriothm](https://eprint.iacr.org/2014/106)), including shaving a $$O(\log n)$$ factor that I will describe at the end of this post.
When measured in $$\mathbb{Z}_q$$ operations, our protocol is within constant factors of the cost of the LWE-based protocol, which is within constant factors of the underlying *plaintext* computation.
While this is great, recall that existing protocols have server compute comperable to a *linear scan* through the database.
So losing constant factors (which I expect to be $$\approx 10\times$$) isn't good enough in this setting.
There are some other limitations of our protocol (it defers certain operations to the client post-decryption, and some more techincal things),
in our setting of PIR it is theoretically quite compelling.

## SimplePIR as a Reduction from PIR to LinPIR

As mentioned in the above, at a high level one can build PIR from LWE by having

1. Clients upload $$C := [A, A\cdot s + f + (q/p)e_i]$$,
2. The server compute $$\mathsf{DB} \cdot C$$, and
3. return this to the client, who decrypts.

This requires transmitting $$A$$ and $$\mathsf{DB}\cdot A$$, which are large.
SimplePIR optimizes this simple protocol by "standardizing" a single matrix $$A := \mathsf{PRG}(sd)$$, and having the client download $$(sd, \mathsf{DB}\cdot \mathsf{PRG}(sd))$$ before making queries.
Modulo certain technicalities, this allows the client to omit transmitting $$A$$ from their query,
and the server to omit transmitting $$\mathsf{DB}\ast A$$, fixing the issue.

There is an alternative way to view what SimplePIR is doing, which I find useful in describing our protocol.
Recall that one way of viewing a standard PIR scheme is as a way of securely computing

$$M, x \mapsto M\cdot x$$

where $$M = \mathsf{DB}$$ is a database, formatted as a matrix, and $$x =e_i$$ is a basis vector.
SimplePIR reduces this to securely computing

$$M, x\mapsto M\cdot x$$

where $$M = \mathsf{DB}\ast A$$ is the "hint", and $$x = s$$ is the LWE secret.
We call this a *linear* PIR scheme.
So SimplePIR reduces a standard PIR query to the database $$\mathsf{DB}\in\mathbb{Z}_p^{\sqrt{m}\times \sqrt{m}}$$
to a linear PIR query to the "hint" $$H:=\mathsf{DB}\cdot A\in \mathbb{Z}_q^{\sqrt{m}\times n}$$.
For large-enough $$m$$ this is a smaller database, yielding a concrete savings.
SimplePIR itself can be viewed as using the trivial PIR protocol of transmitting $$H$$ to the client to answer these linear PIR queries.

Our protocol modifies this to use our RLWE-based PIR scheme to answer these linear PIR queries.
As mentioned before, this (concretely) loses constant factors compared to SimplePIR.
This yields to a scheme that has server computational costs of the form

* $$m + O(\sqrt{m}n)$$ operations in $$\mathbb{Z}_{2^{32}}$$, compared with
* $$m$$ operations in $$\mathbb{Z}_{2^{32}}$$ for SimplePIR.

So, for large-enough $$m$$, we can replace the trivial linPIR query to $$H$$ with a linPIR query to our scheme at no asymptotic cost.
This is (theoretically) the content of the main construction of our paper.
Practically, we use several additional optimizations to make sure  that "large-enough" is not too large.
I'll mention my favorite such optimization below.

# An $$O(\log n)$$ Speedup for many RLWE-based (Linearly) Homomorphic Computations

RLWE-based cryptography often describes computations in terms of two isomorphic rings.
There is

* $$R_{n,q} = \mathbb{Z}_q[x] / (x^n+1)$$, which is a ring under addition and multiplication of polynomials modulo $$(q, x^n+1)$$, and
* $$\widehat{R}_{n,q}$$, e.g. the set $$\mathbb{Z}_q$$ under coordinate-wise addition and multiplication.

These rings are only isomorphic for certain $$q$$ ("NTT-friendly"), but they allow you to replace polynomial multiplication (naively $$O(n^2)$$ time) with a linear-time operation.
One can convert between the two rings at cost $$O(n\log n)$$ operations in $$\mathbb{Z}_q$$.
This cost is often the asymptotically dominant cost in homomorphic computations, to the point that counting how many conversions (NTTs) one uses
is generally a decent proxy for overall running time.

We find a way to precompute all NTTs that occur in our protocol in a way that is independent of things like

* the particular client making a query, and
* the particular database one is issuing a linear PIR query to.

Practically, after executing the protocol once (say during a preprocessing phase) and producing state that may be kept *locally* on the server,
we get an $$O(\log n)$$ speedup.

The high-level idea is simple.
Recall that RLWE-based ciphertexts $$[a(x), b(x)]$$ consist of *pairs* of polynomials, where

* $$a(x)$$ is independent of the message (and should be "random"), and
* $$b(x) = a(x)\ast s(x) + f(x) + (q/p)m(x)$$ is "structured".

One almost always wants to keep polynomials in the evaluation representation $$\widehat{R}_{n,q}$$, as it has more efficient multiplication.
The only time we want to switch back to $$R_{n,q}$$ is to compute operations that depend on particular coefficients of the polynomial, which happens
when multiplying a ciphertext by a large plaintext polynomial with low noise growth.
For simplicity, say that multiplying $$[a(x), b(x)]$$ by the large plaintext polynomial $$d(x)$$ [^polyhom]
produces a ciphertext $$c' = [F_0^d(a), F_1^d(b)]$$.
Our insight is that if one knows $$d$$ beforehand, the maps $$F_0^d, F_1^d$$ can (after preprocessing that is $$O(\log n)$$ slower) be computed in linear time.
Moreover, the map $$F_0^d$$ is *deterministic*, and therefore

1. if you start knowing $$a(x)$$ (say because $$a(x)= \mathsf{PRG}(sd)$$), and
2. also know $$d(x)$$ (which is generally the $$a(x)$$ polynomial of a different ciphertext, for example during key-switching),

then the result is a ciphertext $$c' =[F_0^d(a), F_1^d(b)] = [a'(x), b'(x)]$$ that we also know the polynomial $$a'(x)$$ for.
This leaves one in the position to iterate this argument, and remove all NTTs from (certain) computations.

[^polyhom]: This is not as simple as computing $$[c(x)\ast a(x), c(x)\ast b(x)]$$. This works for "small" polynomials $$c(x)$$, but increases the
            error too much when $$c(x)$$ is large. As I am trying to avoid discussions of error in (R)LWE here, I will solely point interested readers to [this](https://eprint.iacr.org/2020/086).

We haven't tried to describe the precise class of circuits which our preprocessing optimization works for.
My current understanding of the limits are that

* Homomorphic multiplications (either "tensor-based" or "gadget-based") do not preserve our "knowing $$a(x)$$" invariant [^gadget]
* Homomorphic additions work fine
* gadget-based key-switching works fine.

[^gadget]: This is to say that we can preprocess NTTs in a circuit using gadget-based operations until the first gadget-based homomorphic multiplication, after which we can no longer preprocess things.

So we can preprocess complex linearly-homomorphic computations (for example the matrix-vector multiplication protocol in our work), but not things like FHEW-type bootstrapping.
There are also some costs for this optimization.
We require that *the server* knows $$a(x)$$ beforehand.
It is therefore natural to have the server pick the seed $$sd$$, and transmit it to all clients.
This works, but requires the the clients resample an RLWE key $$s(x)$$ for each query to maintain security.
More importantly, they must also resample any encrypted key material.
In our particular setting this comes at low cost, but in other settings it is likely something that must be balanced against
the potential $$O(\log n)$$ speedup.

Anyway, this is skipping plenty of other stuff in our paper (for example, an additional PIR scheme!), but
seems like a decent analogue for putting together some hard-to-read-twitter-thread.

---
