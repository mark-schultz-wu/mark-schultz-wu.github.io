---
layout: post
title: Lattice Coding in Lattice Cryptography
---
Lattice occur within two fields in theoretical computer science --- coding
theory, and cryptography.
Oddly enough, these two fields do not interact that much, and have different
conventions for similar concepts [^smoothing].
For this reason, it can often be useful to look at the conventions of the other
field.
In this post, I will do this, with applications to a class of lattice-based
encryption schemes.

[^smoothing]: (for example, the *smoothing parameter* of a
    lattice was defined in ~2006 within lattice cryptography by Micciancio and
Regev, and appears in a slightly modified way under a *different name* as the "flatness factor" within
Lattice Coding).


# A Class of LWE Encryption Schemes

It is rather simple to build an encryption scheme from the learning with errors
assumption.
This assumption is that (for certain parameters) the learning with errors
problem is hard.

> **LWE Problem**: Let $$n, m, q\in\mathbb{N}$$, and let $\chi$ be a distribution
> on $$\mathbb{Z}_q := \mathbb{Z}/q\mathbb{Z}^m$$. Let $$\vec{s}\gets\mathbb{Z}_q^n$$ Then:
>
> $$(\mathbf{A}, \vec{s}^t\mathbf{A} + \vec{e}) \approx_c (\mathbf{A}, \vec b)$$
>
> Where $$\mathbf{A}\gets \mathbb{Z}_q^{n\times m}$$, and $$\vec b \gets
> \mathbb{Z}_q^m$$

The "for certain parameters" does quite a bit of work here --- it is rather
easy to show that these distributions are *statistically indistinguishable*
under certain parametrizations (namely, let $$\chi$$ be the uniform distribution
on $$\mathbb{Z}_q^m$$).
As we will see later, to build useful cryptography we generally need $$\chi$$ to
be of (comparitively) small support within $$\mathbb{Z}_q^m$$.
In this setting, there are certain parametrizations which are hypothesized to be
hard (generally supported by worst-case to average-case reductions from standard
lattice problems), but I will not be particularly interested in the concrete
hardness of LWE in this post --- just that there exist parameterizations that
are plausibly hard.

This leads us to define the "LWE function":

$$\mathsf{LWE}_{\vec s}(m) = (\mathbf{A}, \vec{s}^t\mathbf{A} + m + \vec e)$$

where $$\mathbf{A}, \vec e$$ are generated as in the LWE problem.
It is rather simple to show that this is (as a symmetric cryptosystem) IND-CPA
secure.
We have that $$(\mathbf{A}, \mathbf{A}\vec s + m + \vec e) \approx_c (\mathbf{A}, \vec b + m)
= (\mathbf{A}, \vec b')$$, where the first transformation is by the hardness of the LWE problem, and the
second is by a variant of the argument of the security of the one-time pad.

The issue with the above is that the scheme is not correct.
A sensible decryption algorithm is given by:

$$\mathsf{LWE}_{\vec s}^{-1}(c) = [-\vec s^t, I]c = [-\vec s^t, I](\mathbf{A}, \vec
b) = -\vec{s}^t\mathbf{A} + \vec{b}$$

Here, I use the convention that $$[A, B]$$ denotes horizontal concatenation of
two appropriately-sized matrices, and $$(A, B)$$ denotes vertical concatenation.
This allows one to write the decryption function as a linear function of the
ciphertext, which is nice.
What isn't nice is that decryption is incorrect.
One can compute that:

$$\mathsf{LWE}_{\vec s}^{-1}(\mathsf{LWE}_{\vec s}(m)) = [-\vec s^t,
I](\mathbf{A}, \vec{s}^t\mathbf{A} + m + \vec{e}) = m + \vec{e}$$

One can fix this by first encoding the message under a particular type of
error-correcting code, and then decoding the error as the final step of
decryption.
As a result, we should probably talk about some coding theory.

# Coding Theory for Lattice Cryptography

The most common way that coding theory shows up in theoretical computer science
is via the notation of binary (forward) error-correcting codes.
These are subsets $$\mathcal{C}\subseteq \mathbb{F}_2^n$$ with certain
properties, namely that they are "well-separated" in a formal sense.
In particular, the (Hamming) distance between any two codewords is lower bounded
by some positive integer (known as the minimum distance of the code), which
allows one to correct a certain number of mistakenly flipped bits.

I find it useful to have a more abstract notion of a code.
In particular, a code has a message space $$\mathcal{M}$$, codebook
$$\mathcal{C}$$, and ambient space $$\mathcal{X}\supseteq \mathcal{C}$$.
Given these sets, a code is a pair of maps:

$$\mathsf{encode} : \mathcal{M}\to\mathcal{C},\quad \mathsf{decode} :
\mathcal{X}\to\mathcal{M}$$

You then usually want these maps to satisfy some other property.
For example, given an admissable (correctable) error region $$\mathcal{E}\subseteq
\mathcal{X}$$, you may want that:

$$\forall m\in\mathcal{M} : \mathsf{decode}(\mathsf{encode}(m) + \mathcal{E}) = m$$

i.e. decoding can correct any admissable error.
I will call any code satisfying such a property an *error-correcting code*, and
will furthermore call it *linear* if $$\mathcal{M}$$ and $$\mathcal{X}$$ are
abelian groups, and $$\mathsf{encode}$$ is a group homomorphism.

If you are unfamiliar with error-correcting codes, feel free to skip this
paragraph.
If not, note that one can recover the standard notion of an $$[n, k, d]$$ binary code as the
above, by setting $$\mathcal{M} = \mathbb{F}_2^k, \mathcal{X} =
\mathbb{F}_2^n$$, and $$\mathcal{E} = \mathcal{B}^n_0(d)$$ to be the Hamming (or
"$$\ell_0$$") ball in $$\mathbb{F}_2^n$$ of distance $$d$$.

Another notion of code we will find useful is that of a *covering code*.
These are codes that are used for lossy compression.
First, I introduce the notion of *rounding* to a code.
Define $$\lfloor x\rceil_{\mathcal{C}} = \mathsf{encode}(\mathsf{decode}(x))$$.
A sensible interpretation of this is that it maps $$x\in\mathcal{X}$$ to the
nearest codeword $$c\in\mathcal{C}$$, although this depends on
$$\mathsf{encode}, \mathsf{decode}$$ being defined sensibly.
I will also define $$x\bmod \mathcal{C} = x - \lfloor x\rceil_{\mathcal{C}}$$ to
be the error induced by this rounding operation.

A covering code is, for some admissable (inducable) error region
$$\mathcal{E}\subseteq \mathcal{X}$$, a code such that $$\mathcal{X}\bmod\mathcal{C}\in \mathcal{E}$$
In particular, covering codes can compress any point in ambient space to a
codeword (which may have smaller representations) while inducing bounded error.

As mentioned before, given an error-correcting code, one can define:

$$\mathsf{Enc}_{\vec s}(m) = \mathsf{LWE}_{\vec s}(\mathsf{encode}(m)),\quad
\mathsf{Dec}_{\vec s}(c) = \mathsf{decode}(\mathsf{LWE}_{\vec s}^{-1}(c))$$

provided parameters are chosen correctly (in particular, that the
error-correcting code can correct errors from the distribution $$\chi$$ from the
LWE problem), this gives a perfectly correct encryption scheme, which by our
prior discussion is IND-CPA secure.

With this motivation, there's a natural question --- how does one *find* an
error-correcting code of suitable parameters?
Funnily enough, a (relatively standard) way to construct an error-correcting
code for use in lattice cryptography is via lattices themselves.

## The Coding-theoretic Interpretation of the Hardness of LWE

There is a standard coding-theoretic interpretation of the hardness of LWE.
In particular, an LWE sample $$(\mathbf{A}, \mathbf{A}\vec s + \vec e)$$ can be
interpreted as the encoding of the secret key $$\vec s$$ by the random linear
code $$\mathbf{A}$$.
This is then sent through the noisy channel $$X\mapsto X + \vec e$$, which an
adversary then has to decode to recover $$\vec s$$.
Random linear codes actually have quite good parameters, so it would be quite
useful if we were able to decode them.
Of course, the above story says nothing about which parameters the LWE problem
is easy/hard for, so isn't technically worth much.

Still, this connects the "basic" component of LWE encryption to the *channel
coding* problem (which is the problem of finding good error-correcting codes),
not the *source coding* problem (the problem of finding good covering codes).

# Lattices and Codes

Slightly simplistically [^simplistic], a lattice is a discrete subspace
$$L\subseteq\mathbb{R}^n$$. 
Any lattice can be written as $$L = \mathbf{B}\mathbb{Z}^k$$ where
$$\mathbf{B}\in\mathbb{R}^{n\times k}$$, where $$\mathbf{B}$$ is known as a
*basis* of the lattice, and $$k$$ is known as the *rank*. Not every choice of
$$\mathbf{B}$$ leads to a lattice (as $$\mathbf{B}\mathbb{Z}^k$$ will not always
be discrete). We will restrict to lattices which are full-rank, meaning where
$$k = n$$.

[^simplistic]: This is simplistic as mathematically a lattice can be a "discrete subspace" of a
    different space, and the space has to come with some notion of distance (otherwise abstractly as groups
    all lattices are isomorphic to $$\mathbb{Z}^k$$). Mathematically one often
    specifies a quadratic form, so lattices are a type of "Quadratic Space" (similar
    to how linear algebra studies linear spaces). One can instead specify lattices
    as a discrete subspace with some associated metric. These nuances will not be
    important.

One should view a lattice $$L\subseteq \mathbb{R}^n$$ as analougous to a *linear* code
$$\mathcal{C}\subseteq \mathbb{F}_2^n$$. The "$$\mathbb{R}$$ analogue" of a binary error-correcting
code is commonly known as a "sphere packing". A sphere packing is a subset
$$\mathcal{S}\subseteq \mathbb{R}^n$$ such that one can (for some fixed radius $$r$$)
place *non-overlapping* spheres $$\mathcal{B}_2^n(r)$$ around each point in $$\mathcal{S}$$.
In some dimensions [^dimensions] it is known that non-lattice packings are
better (under a certain notion of density) than lattice packings.
Apparently any non-lattice packing can be approximated by "almost" lattice
packings (writing it as a union of cosets of lattice packings), but I have only
seen this statement quoted, and haven't seen a proof.

[^dimensions]: Lattice packings are an area of math where results are *highly*
    dependent on the dimension under consideration. For example, optimal sphere
    packings are known in dimensions $$\{1,2,3,8,24\}$$, due to quite different
    constructions being possible in dimensions 8 and 24 (the Gosset and Leech
    lattices). These constructions are independently interesting, are connected
    to "extremal" coding-theoretic constructions (the Hamming and Golay codes),
    and to sporadic simple groups.

## Construction A

There are a family of well-known ways to convert between lattices and codes
(often called "Constructions", for example Construction A, B, C, D, E, etc).
The simplest relates a binary code $$\mathcal{C}\subseteq \mathbb{F}_2^n$$ and a
lattice $$L\subseteq \mathbb{R}^n$$.
There is a map in each direction:

$$\mathcal{C}\mapsto \mathcal{C} + 2\mathbb{Z}^n,\quad  L \mapsto L\bmod 2$$

One can extend this to $$q$$-ary codes (meaning subsets of $$\mathbb{Z}_q$$) in
a rather straightforward way.
The first of these sends $$\mathcal{C}\subseteq \mathbb{Z}_q^n$$ (which one can view as a point cloud
within the box $$\lbrack-q/2, q/2)^n\cap \mathbb{Z}^n$$) to the set $$\mathcal{C} +
q\mathbb{Z}^n$$. One can interpret this as "copying" the point cloud around to the points of
$$q\mathbb{Z}^n$$.
The reverse direction can be thought of as "merging" all of the boxes
together.

Within lattice cryptography, one often works with the following lattices (where
$$\mathbf{A}\in\mathbb{Z}_q^{d\times n}$$):

$$\Lambda_q(\mathbf{A}) = \{\vec x\in\mathbb{Z}^n \mid \exists x\in\mathbb{Z}^k
: \mathbf{A}\vec{x}\equiv \vec x\bmod q\},\quad \Lambda_q(\mathbf{A})^\perp =
\{\vec y \in\mathbb{R}^n\mid \mathbf{A}\vec y\equiv 0\bmod q\}$$

These are the "$$q$$-ary image" and "$$q$$-ary kernel" of the code.
These are dual in a formal sense (see [these
slides](https://cseweb.ucsd.edu/~daniele/papers/DualitySlides.pdf)) that we will
not need.

# A High-Rate Encryption Scheme

LWE ciphertexts are of the form $$(\mathbf{A}, \vec b := \mathbf{A}\vec s +
\mathsf{encode}_E(m) + \vec e)$$, where $$E$$ is an error-correcting code.
If one wants these ciphertexts to be smaller, there is a quite silly thing that
you can do --- compress them! Of course, you can't help for lossless compression
(this would contradict the pseudorandomness of the LWE distribution), but lossy
compression could still *potentially* work, as we're *already* wrapping our
message in an error-correcting code.

Oddly enough, this idea precisely works!
In particular, if one lets $$Q$$ be some covering code, then the encryption
scheme:

$$\mathsf{Enc}_{\vec s}(m) = (\mathbf{A}, \mathsf{decode}_Q(\mathsf{LWE}_{\vec s}(\mathsf{encode_E(m)})),\quad \mathsf{Dec}_{\vec s}(c) = \mathsf{decode}_E(\mathsf{LWE}_{\vec s}^{-1}(\mathsf{encode}_Q(c)))$$

works under quite reasonable conditions.
I didn't mention the parameters you have to set before for correctness, but it
isn't difficult to check that if $$\vec e\gets \chi$$ is the error distribution,
then ones that $$\mathsf{supp}(\chi)\subseteq \mathcal{E}$$, where
$$\mathsf{E}$$ is the allowable error region of $$E$$.
In this new construction, you have to correct quantization error as well --- so
you need that $$\mathsf{supp}(\chi) + \mathcal{Q}\subseteq \mathcal{E}$$, where
$$\mathcal{Q}$$ is the inducable error region of $$Q$$.
The proof of correctness under these conditions is straightforward (although I
don't have as elegant of a way to write it notationally as I might want).
One starts by writing out:

$$
\mathsf{Dec}_{\vec s}(\mathsf{Enc}_{\vec s}(\vec m)) =
(\mathsf{decode}_E\circ\mathsf{LWE}_{\vec s}^{-1}\circ\mathsf{encode}_Q\circ
\mathsf{decode}_Q\circ\mathsf{LWE}_{\vec s}\circ \mathsf{encode}_E)(\vec m)
$$

Then you "pair" up things iteratively, i.e. replace the $$\mathsf{encode}_Q\circ\mathsf{decode}_Q$$
functions with the function $$\vec x\mapsto \lfloor \vec x\rceil_Q$$ (that's the
definition), which you can then write as $$x - x\bmod Q = x - \mathcal{Q}$$.
Repeating this, you end up with $$\mathsf{decode}_E(\mathsf{encode}_E(\vec m) +
\mathsf{supp}(\chi) - \mathcal{Q})$$, i.e. get correct decoding if
$$\mathsf{supp}(\chi) + (-\mathcal{Q})\subseteq \mathcal{E}$$.
Interestingly, you don't need that your codes are nested, or even linear.
Assuming both of these hold though, you can write encryption in a rather nice
form:

$$\mathsf{Enc}_{\vec s}(\vec m) = (\mathbf{A}, T\vec m +
\mathsf{decode}_Q(\mathsf{LWE}_{\vec s}(0)))$$

Here, $$T$$ is a certain integer matrix that "relates" (the generator matrices
of) two nested lattice that I will not discuss.
This is a rather nice representation of things though, because it makes it
*obvious* that after this compression operation one is still lefted with an
encryption scheme admitting some degree of additive homomorphism.

Interestingly, this simple technique of compressing the ciphertext is enough to
get one a higher rate scheme than before.
In fact, one can get rates that were *impossible* before, which is quite
interesting to think about.
