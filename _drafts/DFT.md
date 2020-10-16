---
layout: post
title: Gadget Matricies and the Uncertainty Principle
---

Lattice-based cryptography has a variety of *slightly* different encryption
schemes.
In this post I'll discuss two common ones (the initial ["scaling" scheme due to Regev](https://cims.nyu.edu/~regev/papers/qcrypto.pdf), and a
scheme which uses the [Micciancio-Piekert "gadget" matrix](https://eprint.iacr.org/2011/501)), and show how they
can be formally connected via the Discrete Fourier transform.
Moreover, I'll discuss how a key property of the "gadget" matrix (that it has
unusually "short" preimages),
can be interpreted as a consequence of (a generalized form of) the Heisenberg uncertainty principle.

This post was getting fairly long, so I decided to split the writeup into two posts.
See this post for background material
on lattices and the Learning with Errors problem.

## The Encryption System

Recall that the LWE problem is about distinguishing the probability
distributions $$(\mathbf{A},
\mathbf{A}\vec{s} + \vec{e})$$ and $$(\mathbf{A}, \vec{u})$$.
In particular, if the LWE problem is hard, then these distributions are
computationally indistinguishable (notated $$\approx_c$$).
This leads to a *very* simple encryption scheme --- use $$\vec{u}$$ as a one-time pad to encrypt $$\vec{m}$$.
In particular, let $$\vec{s}$$ be the secret key, and define:

$$\begin{aligned}
\mathsf{Enc}_{\vec s}(\vec m) &= (\mathbf{A}, \mathbf{A}\vec{s} + \vec{e} +
\vec{m})\\
\mathsf{Dec}_{\vec s}(\mathbf{A}, \vec{b}) &= \vec{b} - \mathbf{A}\vec{s}
\end{aligned}
$$

This encryption scheme is not yet correct though --- in particular:

$$

\begin{aligned}
\mathsf{Dec}_{\vec s}(\mathsf{Enc}_{\vec s}(\vec m)) &= (\mathbf{A}\vec{s} +
\vec{e} + \vec{m}) - \mathbf{A}\vec{s}\\
&= \vec{m} + \vec{e}
\end{aligned}
$$

We can fix this by using an *error-correcting code*.
I will discuss these more in the next section, but in short these are a pair of
mappings $$(\mathsf{encode}(\cdot), \mathsf{decode}(\cdot))$$ such that
for some set of decodable errors $$\mathcal{E}$$:

$$\forall e\in \mathcal{E} : \mathsf{decode}(\mathsf{encode(m)} + e) = m$$

Traditional (binary) error-correcting codes are usually defined where
$$\mathcal{E}$$ is some ball in the *Hamming metric*.
Error-correcting codes for the $$\ell_2$$-metric can naturally be built out of
lattices (again, as I will discuss in the next section).

Anyway, given an error-correcting code $$(\mathsf{encode}(\cdot), \mathsf{decode}(\cdot))$$ with decodable error region $$\mathcal{E}\supseteq \mathsf{supp}(\chi^m)$$, it is rather straightforward to show that:

$$
\begin{aligned}
\mathsf{Enc}'_{\vec s}(\vec m) &= \mathsf{Enc}_{\vec s}(\mathsf{encode}(\vec m))\\
\mathsf{Dec}'_{\vec s}(\mathbf{A}, \vec b) &= \mathsf{decode}(\mathsf{Dec}_{\vec
s}(\mathbf{A}, \vec b))
\end{aligned}
$$

is perfectly correct.
Security is quite easy to show for this.
The argument proceeds as:

$$
\begin{aligned}
\mathsf{Enc}'_{\vec s}(\vec m) &= (\mathbf{A}, \mathbf{A}\vec{s} + \vec{e}) +
(0, \mathsf{encode}(\vec m))\\
&\approx_c (\mathbf{A}, \vec b) + (0, \mathsf{encode}(\vec m))\\
&\approx_s (\mathbf{A}, \vec b')
\end{aligned}
$$

Here, $$\approx_c$$ follows from the hardness of the LWE problem, and
$$\approx_s$$ follows from the same argument as the
one-time pad.
It follows that ciphertexts are computationally indistinguishable from random,
and the scheme is semantically secure.


# The Scaling Code and the Gadget Matrix

Traditionally, one builds such codes in an ad-hoc way.
A common source of them are what you might call *lattice codes*.
Given a basis $$\mathbf{B}$$ for the lattice $$L$$, you can encode $$\vec{m}$$ by computing
$$\mathbf{B}\vec{m}\in L$$.
As mentioned before, each lattice point $$\ell\in L$$ is the center of a ball of
radius $$\lambda_1(L)$$ which contains no other lattice points.
If we shrink these balls down to radius $$\lambda_1(L)/2$$, one can further show that
all balls are *disjoint*.
This means that given any lattice point $$\ell\in L$$, provided $$\lvert\vec
e\rvert_2
\leq \lambda_1(L)/2$$, that $$\ell + \vec{e}$$ is still in the sphere centered
at $$\ell$$.
So one can construct error-correcting codes from lattices!

There are a few properties that you want for this to work:

1. Encoding to be efficient (it can often be represented as $$\mathsf{encode}(\vec m) = \mathbf{A}\vec m$$ easily, so reduces to a matrix-vector product)
2. Decoding to be efficient (this is more difficult)
3. $$\lambda_1(L)$$ to be large, while the lattice is still "dense"

The third point has to do with how much you have to "pay" (in terms of the
size of the encoding) to get this error resiliance.
The second point is additionally pretty important --- decoding as we've
described it is known as the *closest vector problem*, which is
$$\mathsf{NP}$$-complete for general lattices.
One has to make sure to pick lattices that you can efficiently decode, meaning
for which CVP (or some appropriately relaxed version of it) is efficient.
I'll next describe an *extremely* simple lattice for which this is true.

## The Scaling Code

Let $$2\mid q$$ (but the generalization to any $$p\mid q$$ is not that much more
difficult).
Define the functions:

$$
\begin{aligned}
\mathsf{encode}_S(m) &= (q/2)m \bmod q\\
\mathsf{decode}_S(x) &= \lfloor(2/q)x\rceil
\end{aligned}
$$

Here, "S" stands for "Scaling", and $$\lfloor\cdot\rceil$$ denotes rounding to the nearest integer.
This code can encode messages in $$\{0, 1\}$$, and has "ambient space" (meaning
the codomain of $$\mathsf{encode}$$, and domain of $$\mathsf{decode}$$)
$$\mathbb{Z}_q$$.
It encodes messages onto the lattice[^1] $$\{0, q/2\}$$ --- for $$p = 2$$,
this is simply $$\{0, q/2\}\subseteq \mathbb{Z}_q$$.
One can show that $$\lfloor x + e\rceil = x$$ for $$x\in\mathbb{Z}, e\in[-1/2, 1/2)$$.
This is sufficient to show that:

[^1]:Astute readers will note that $$L = \{0, q/2\}$$ cannot be a lattice, as
    $$q/2 + q/2 = q\not\in L$$. This is because we are interpreting $$L$$ as a
    $$q$$-ary
    lattice (of *course*), so when discussing the "actual lattice" we look at
    $$L + q\mathbb{Z} = (q/2)\mathbb{Z}$$, which *is* a lattice (or simply do
    all of
    our operations $$\bmod\ q$$).
      
    Essentially we are converting between viewing $$L$$ as a
    $$\mathbb{Z}_q$$-module and a $$\mathbb{Z}$$-module (which is the same
    thing as an abelian group).
    This is the kind of thing which is both standard, and I believe isn't
    described well enough in most places (maybe I'll write something about it
    eventually), especially as one is must *also* place a different metric on $$L$$
    when it is viewed as a $$\mathbb{Z}_q$$-module (an $$\ell_2$$ variant
    of the so-called Lee metric).


$$
\begin{aligned}
\mathsf{decode}(\mathsf{encode}(m) + e) &= \lfloor (2/q)((q/2)m + e)\rceil\\
&= \lfloor m + (2/q)e\rceil
\end{aligned}
$$

This correctly decodes if $$(2/q)e\in [-1/2, 1/2)\implies e\in[-q/4, q/4)$$.
It's worth mentioning that the size of the message to be encoded is
$$\log_2|\{0,1\}| = 1$$, and our encoding is of size $$\log_2|\mathbb{Z}_q| =
\log_2 q$$.
This means the "rate" of our scheme is $$\log_2 2/\log_2 q = 1/\log_2 q$$.
It's not hard to show that the rate is always below 1 if your encoding is injective (and therefore you can decode in
the presence of no errors), and higher rate is better.

With that analysis done (I mentioned it was simple!), we move onto the next
code.

## The Gadget Matrix Code

Another common code is the so-called "gadget matrix" code.
This is somewhat more complicated, but has a nice connection to the scaling code
which I hope to elaborate below.

Let $$k\in\mathbb{N}$$, and let $$q = 2^k$$.
Note that $$2\mid q$$ again (our setup from before), and I quickly mention that
we can again extend this to the setting of more general $$p\mid q$$.

Let $$\vec g = (1,2,\dots,2^{k-1})$$.
The idea is now to encode $$m\in\mathbb{Z}_q$$ by *scalar multiplying* it by
$$\vec g$$:

$$
\begin{aligned}
\mathsf{encode}(m) &= m\vec g\bmod q
\end{aligned}
$$

One can check that $$m\vec g\bmod q = (m, 2m, 4m, \dots, 2^{k-1}m)\bmod q$$.
Say that some error $$\vec e\in\mathbb{Z}_q^k$$ gets added to this.
Let $$\vec c = m\vec g + \vec e\bmod q$$, and let $$\vec c_i = 2^im + e_i\bmod
q$$ be the $$i$$th component of $$c$$.
One can write 
$$m = \sum_{i = 0}^{k-1} m_i2^i$$ in base 2.
Then we have that:

$$
\begin{aligned}
\vec{c}_{k-1} &= 2^{k-1}m + \vec e_{k-1}\bmod q \\
&= 2^{k-1}m_0 + 2^k(\sum_{i = 1}^{k-1}m_i2^{i-1}) + \vec e_{k-1}\\
&\equiv (q/2)m_0 +\vec{e}_{k-1}\bmod q
\end{aligned}
$$

We can therefore view the component $$\vec{c}_{k-1}$$ as being the encoding of
$$m_0$$ under the
*scaling code*.
It follows that we can compute $$\vec m_{0} = \mathsf{decode}_S(\vec c_{k-1})$$, and can
tolerate error $$\vec{e}_{k-1}\in[-q/4, q/4)$$.
We can then examine:

$$
\begin{aligned}
\vec{c}_{k-2} &= 2^{k-2}m + \vec{e}_{k-2}\bmod q\\
&= 2^{k-2}m_0 + 2^{k-1}m_1 + \vec{e}_{k-2}\bmod q\\
&= (q/2)m_1 + \vec{e}_{k-2} + 2^{k-2}m_0\bmod q
\end{aligned}
$$

Of course, we already know $$m_0$$, so we can simply compute $$\vec{m}_1 =
\mathsf{decode}_S(\vec{c}_{k-2} - 2^{k-2}m_0)$$.
We can again tolerate error $$\vec{e}_{k-2}\in[-q/4, q/4)$$, precisely the same
as the scaling code.
One can continue this process iteratively, to derive the general rule:

$$\vec{m}_i = \mathsf{decode}_S\left(\vec{c}_{k-1-i} - \sum_{j = 0}^{i-1} 2^{k-1-j}\vec{m}_j\right)$$

It follows from straightforward computations[^straightforward] after you've
observed the "general rule" above.

[^straightforward]: I've included the computations below not because I find them
    particularly interesting, but because I wrote them out, so might as well keep
    them somewhere. It was either here or commented out in the source, and I view
    the two as being roughly equivalent.
    $$
\begin{aligned}
\vec{c} &= m(1,2,\dots,2^{k-1}) + \vec{e} \bmod q\\
\implies \vec{c}_{k-1-i} &= 2^{k-1-i}m + \vec{e}_{k-1-i}\bmod q\\
&= 2^{k-1-i}\sum_{j = 0}^{k-1}2^jm_j + \vec{e}_{k-1-i}\bmod q\\
&= \sum_{j = 0}^{k-1}2^{k-1-i+j}m_j + \vec{e}_{k-1-i}\bmod q\\
&= \sum_{j = 0}^{i}2^{k-1-i+j}m_j + \vec{e}_{k-1-i}\bmod q\\
&= (q/2)m_i + \sum_{j = 0}^{i-1}2^{k-1-i+j}m_j + \vec{e}_{k-1-i}\bmod q\\
\implies m_i &= \mathsf{decode}_S\left(\vec{c}_{k-1-i} - \sum_{j=0}^{i-1}2^{k-1-i+j}m_j\right)
\end{aligned}
$$

One can interpret this iterative process quite naturally via
*backsolving a system of linear equiations*.
To explore this more, we'll first examine a natural generalization of the Gadget
matrix construction --- explicitly decomposing $$m$$ in base-2 first.




, showing that we can tolerate
$$\vec{e}\in [-q/4, q/4)^k$$.
Moreover, the rate of this is $$\log_2 q/\log_2 q^k = 1/k = 1/\log_2 q$$.
This has the same parameters as $$k$$ copies of the scaling code.
Here, we mean the *direct sum* of $$k$$ copies of the 1D lattice $$L =
(q/)2\mathbb{Z}$$.
This just ends up being $$(q/2)\mathbb{Z}^k$$.

(formally, the
$$k$$-fold direct sum with itself, or the tensor product with the $$I_k$$
identity matrix).
So this codes has the same parameters of $$k$$ copies of the scaling code ---
what gives?

## The Vandermonde Matrix and the DFT

Recall the definition of the [Vandermonde
Matrix](https://en.wikipedia.org/wiki/Vandermonde_matrix).
Given a vector $$(\alpha_1,\dots, \alpha_n)$$, it is the matrix:

$$
V = \begin{pmatrix}
1 & \alpha_1 & \alpha_1^2 & \dots & \alpha_1^{n-1}\\
1 & \alpha_2 & \alpha_2^2 & \dots & \alpha_2^{n-1}\\
1 & \alpha_3 & \alpha_3^2 & \dots & \alpha_3^{n-1}\\
\vdots & \vdots & \vdots & \ddots & \vdots \\
1 & \alpha_n & \alpha_n^2 & \dots & \alpha_n^{n-1}
\end{pmatrix}
$$

The Vandermonde matrix is the matrix of *polynomial interpolation*.
Let $$k$$ be a field.
The vector space $$P_n$$ of polynomials (with coefficients in $$k$$) of degree
strictly less than $$n$$ has dimension $$n$$^[^polydim].
So does the vector space $$k^n$$, so they are isomorphic (by a linear map of
course!).
That linear map is precisely $$V$$.
In particular, if we identify a polynomial $$p(x) = \sum_{i = 0}^{n-1}a_i x^i$$
by its vector of *coefficients* (which we will denote $$\vec{p}$$), then one can
check that:

[^polydim]: One basis is $$\{1,x,x^2,\dots,x^{n-1}\}$$, in case you may have
    thought the dimension was $$n+1$$.

$$V\vec p = (p(\alpha_1), p(\alpha_2),\dots, p(\alpha_n))$$

So the Vandermonde matrix converts between a representation of a polynomial in
terms of its *coefficients*, and a representation in terms of its *evaluation*
on $$n$$ distinct points.
As mentioned before, these spaces are isomorphic (as vector spaces at least) for
abstract reasons --- all vector spaces over $$k$$ of dimension $$n$$ are
isomorphic.
The Vandermonde matrix can be interpreted as being the *explicit isomorphism*
between $$P_n$$ and $$k^n$$.

How is this related to the Gadget Matrix code?
Consider the Vandermonde matrix defined by the evaluation points $$(1, 2, \dots,
2^{k-1})$$.
Encode $$m$$ in the *second* slot of a vector (if we did the *first* slot, we
would get back $$(m,m,\dots,m)$$), so $$m\mapsto (0,m,\dots,0)$$.
Then, consider the mapping:

$$V(0,m,\dots,0)^t =(m, 2m, 4m,\dots,2^{k-1}m)^t$$

This is *precisely* encoding via the gadget matrix!
We can therefore see (encoding at least) as being defined in terms of a
particular isomorphism of the underlying vector space[^vectorspace].
What does decoding look like though?

### Gadget Encoding Via Base-2 Decomposition

## Below seems bad

The particular Vandermonde matrix can be written as:

$$
V = \begin{pmatrix}
1 & 1 & 1 & \dots & 1 \\
1 & 2& 2^2 & \dots & 2^{k-1}\\
1 & 2^2 & 2^4 & \dots & 2^{2k-1}\\
\vdots & \vdots & \vdots & \ddots & \vdots\\
1 & 2^{k-1} & 2^{2(k-1)} & \dots & 2^{(k-1)(k-1)}
\end{pmatrix}
$$

In particular, the $$(i, j)$$ term is $$2^{(i-1)(j-1)}$$.
When we consider this $$\bmod 2^{k}$$, this will be 0 precisely when
$$(i-1)(j-1)\geq k$$, or in particular when $$j \geq \lceil\frac{k}{i-1}\rceil
+1$$.
Looking at a small explicit example, let $$k = 5$$.
Then, we have that:

$$
\begin{aligned}
V &= \begin{pmatrix}
1 & 1 & 1 & 1 & 1\\
1 & 2 & 4 & 8 & 16\\
1 & 4 & 16 & 0 & 0\\
1 & 8 & 0 & 0 & 0\\
1 & 16 & 0 & 0 & 0
\end{pmatrix}
\end{aligned}

So $$V$$ isn't *precisely* upper triangular
$$



[^vectorspace]: Here, "overlying" is maybe a better term, as a lattice $$L$$ is
    *contained* in some vector space $$V$$. There's a natural way of converting
    from some lattice to this "overlying vector space" known as the [extension
    of
    scalars](https://en.wikipedia.org/wiki/Change_of_rings#Extension_of_scalars).
    This has a quite general definition in terms of general modules (remember,
    Euclidean lattices are $$\mathbb{Z}$$-modules).
    In our (simple) case, one gets the object:

    $$ L_{\mathbb{R}} = L\otimes_{\mathbb{Z}}\mathbb{R}$$

    Here, $$\mathbb{Z}$$ is the *tensor product*.
    Abstractly, one represents elements of $$L_{\mathbb{R}}$$ via a pair of
    elements $$(\ell, r)\in L\times \mathbb{R}$$, written $$\ell\otimes r$$.
    These satisfy the rule $$(\ell\otimes r) = r(\ell\otimes 1)$$, so it's often
    best to informally write $$\ell\otimes 1\to \ell$$, and view
    $$L_{\mathbb{R}}$$ as "$$L$$ with scalar multiplication in $$\mathbb{R}$$",
    precisely as one wants it to be.



