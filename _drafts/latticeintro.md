---
layout: post
title: A Quick Intro to Lattices
---

The following is a quick "primer" on lattices, mostly as supplementary material
for my first post, although it's quite possible I will
link to this in future posts as well.
Realistically, there are *many* better introductions to lattices than this post
(although they are longer --- probably for good reason!).
For this reason I'll include links to [Daniele Micciancio's Lattice Algorithms course
syllabus](http://cseweb.ucsd.edu/classes/fa17/cse206A-a/), as well as [Chris
Piekert's *A Decade of Lattice
Cryptography*](https://web.eecs.umich.edu/~cpeikert/pubs/lattice-survey.pdf).
There are *many* other resources than just these, but collecting the other
resources would probably be deserve an *entirely separate* post.

# A List of Lattice Facts

While lattice-based cryptography can be defined over a variety of fancy
algebraic structures (and I might discuss some of them on this blog later!), we'll focus
on the down-to-earth case of lattices over $$\mathbb{Z}$$. Before describing the
cryptographic context of this post, I'll quickly do a quick (general)
introduction to Euclidean lattices  (which are not to be confused with
combinatorial lattices, meaning posets which have their meets and joins).

A lattice is a *discrete* subgroup $$L\subseteq \mathbb{R}^n$$.
Here, "discrete" can be defined in many ways, but the easiest for our purposes
will be that the quantity:

$$\lambda_1(L) = \min_{\ell \in L\setminus \{0\}} \lVert \ell\rVert_2$$

is strictly larger than zero.
This quantity is known as the *shortest vector* of $$L$$, and is a quantity
of fundamental importance throughout the study of lattices.
Importantly, this quantity being non-zero implies that *every* lattice point
$$\ell\in L$$ has an $$\ell_2$$-ball of radius $$\lambda_1(L)$$ which contains
no other lattice points.
This is because if the ball centered at $$\ell_0$$ contained $$\ell_1$$, then
$$\ell_0 - \ell_1$$ would be a lattice point of norm strictly less than
$$\lambda_1(L)$$.

It's worth mentioning that the choice of *some* norm (here $$\lVert\cdot\rVert_2$$) is
central to the study of lattices --- without any notion of distance, any
discrete subgroup $$L$$ is isomorphic (as an abstract group) to $$\mathbb{Z}^k$$
for some $$k>0$$, so the study of such groups becomes much less interesting.

One can show that any discrete subgroup $$L$$ of $$\mathbb{Z}^n$$ takes the
form:

$$L = \mathbf{B}\mathbb{Z}^k,\quad \mathbf{B}\in\mathbb{Z}^{n\times k}$$

Where $$\mathbf{B}$$ is known as the *basis* of $$L$$, $$k$$ is the *rank* of
$$L$$, and $$n$$ is the *dimension* of $$L$$.
If $$n = k$$ the lattice is known as *full-rank*.
Such lattices are known as *integral* lattices, and are the most common form of
lattices within lattice cryptography.
One can define lattices within $$\mathbb{R}$$ by having
$$\mathbf{B}\in\mathsf{Mat}_{n\times k}(\mathbb{R})$$, but you have to take
extra care (as the subgroup $$\mathbf{B}\mathbb{Z}^k$$ may not be discrete).

Lattices are intrinsically *periodic* objects.
For any collection of lattice points $$\mathcal{C}\subseteq L$$, and any vector
$$\ell\in L$$, the shifted collection of points $$\ell + \mathcal{C}$$ is also
contained in $$L$$.
A particularly interesting case is that of lattices such that
$$q\mathbb{Z}^n\subseteq L$$.
For such lattices, any element $$\ell\in L$$ has a unique translate $$\ell\bmod
q \in [-q/2, q/2)^n\cap L$$.
We call such lattices $$q$$-ary.
Note that all lattices satisfy this property for *some* $$q$$ (in particular, $$L = \mathbf{B}\mathbb{Z}^k$$ is always $$\det(\mathbf{B}\mathbf{B}^t)$$-ary), but $$q$$-ary lattices are periodic modulo a much *smaller* number than one would expect.
In such lattices, one can often replace all arithmetic with arithmetic in the
ring $$\mathbb{Z}_q :=\mathbb{Z}/q\mathbb{Z}$$, which is useful for computation.
Note that $$q$$ is often *not* prime, so $$\mathbb{Z}_q$$ is not a
finite field, and we are still not doing linear algebra.

All of the above means that working with lattices in practice can often look
similar to linear algebra, although in a slightly "less nice" setting.
I've already mentioned above that in practice people often work over the rings
$$\mathbb{Z}_q$$ rather than $$\mathbb{F}_p$$, but there are *many*
more examples.
One particularly illustrative one is that if $$L\subseteq L'$$, and $$\mathsf{rank}(L) = \mathsf{rank}(L')$$,
this does not imply that $$L = L'$$ (consider the example of the 1D lattices
$$L = \mathbb{Z}$$ and $$L' = 2\mathbb{Z}$$).
This implication of course holds for vector (sub)spaces.
One can learn the pitfalls to avoid eventually, but until one does there can be
unfortunately many pitfalls.
This is one way that I've found many introductions to lattice cryptography
lacking, and may want to write further on in the future --- by emphasizing what
they *aren't* (in particular, which properties hold for vector spaces, but fail
for lattices).

# The Learning with Errors Problem

While there are *many* potential hardness assumptions on lattices, the most
commonly used within cryptography is *by far* the Learning with Errors problem.

> Let $$q, n, m\in\mathbb{N}$$, and let $$\chi$$ be a distribution on
> $$\mathbb{Z}_q$$.
> The $$\mathsf{LWE}_{n, m, \chi, q}$$ problem is to distinguish between the
> distributions:
>
> $$(\mathbf{A}, \mathbf{A}\vec{s} + \vec{e})\approx (\mathbf{A}, \vec{u})$$
>
> Where $$\mathbf{A}\gets \mathbb{Z}_q^{m\times n}, \vec{u}\gets
> \mathbb{Z}_q^m, \vec{s}\gets \mathbb{Z}_q^n$$ are sampled uniformly at random, and $$\vec{e}\gets\chi^m$$

It's worth mentioning that $$\chi$$ must be a concentrated distribution for this
to be interesting --- for example, if $$\chi$$ is uniform on $$\mathbb{Z}_q$$,
the distributions can be shown to be distributed identically.
Before proceeding, I'll quickly mention the following (which is not strictly necessary
for the rest of this blog post):

1. Yes, there are a *lot* of parameters. Setting parameters incorrectly is an
   unfortunately easy mistake to make.
   To find "secure" parameters, it's probably best to refer to the [LWE
   estimator](https://bitbucket.org/malb/lwe-estimator/src/master/) or the
   [Homomorphic Encryption Standard](https://homomorphicencryption.org/standard/)

2. If you solely compare $$\mathbf{A}\vec{s} + \vec{e}$$ with $$\vec{u}$$,
   one can use the Leftover Hash Lemma to show that the two distribution (under
   appropriate settings of parameters) are *statistically close*.
   As we will see later, the random matrix $$\mathbf{A}$$ is key to this being a
   computational problem.

Before applying this to build a simple private-key encryption algorithm, I will
quickly discuss a few "interpretations" of what the LWE is asking you to solve.
These are most naturally presented for the search LWE problem (where one knows
the sample is of the form $$(\mathbf{A}, \mathbf{A}\vec{s} + \vec{e})$$, and
must recover $$\vec{s}$$).

## Heuristic Reasons why Search-LWE is hard

Before discussing these "interpretations", I should mention that the *primary*
reason to believe in the hardness of LWE are the worst-case to average-case reductions
(this is in fact part of the reason that Regev won his G&ouml;del prize).
Still, the expression $$\mathbf{A}\vec{s} + \vec{e}$$ has the following two
different interpretations, one of which will be useful for us later.

### Solving Noisy Equations

Each row of $$\vec{b} = \mathbf{A}\vec{s} + \vec{e}$$ takes the form $$\vec{b}_i = \langle \vec{a}_i, \vec{s}\rangle + \vec{e}_i$$.
Given such a collection of equations, you then must "solve" for the unknown
$$\vec{s}$$.
Without the presence of the noise, one could easily do this with Gaussian
elimination.
Unfortunately (or fortunately, depending on your perspective), Gaussian
elimination behaves poorly with respect to noise (I've heard it explained as it
being numerically unstable before).
So the problem is still hard.

### Decoding Random Linear Codes

The matrix $$\mathbf{A}$$ defines a lattice.
$$\mathbf{A}\vec{s} + \vec{e}$$ can be interpreted as encoding the secret
$$\vec{s}$$, and then passing it through a noisy channel (which adds the noise
$$\vec{e}$$).
In this way, recovering $$\vec{s}$$ from $$(\mathbf{A}, \mathbf{A}\vec{s} +
\vec{e})$$ can be intepreted as decoding a random linear code.
This is known to be hard in the worst case, and is difficult in practice as well
(unlike other $$\mathsf{NP}$$-hard problems such as SAT, which can be easy on
random instances from "natural" distributions).
This is again unfortunate (or fortunate), as these random codes have quite good
parameters.


