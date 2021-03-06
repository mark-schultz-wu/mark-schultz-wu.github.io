---
layout: post
title: Continuous and Discrete Prime Lattices 
---

Today, a [paper was posted on eprint](https://eprint.iacr.org/2021/232) claiming
to be authored by [Schnorr](https://en.wikipedia.org/wiki/Claus_P._Schnorr), and
claiming (in the abstract at least) to break RSA.
The paper itself appears to have been a work-in-progress (a version of it made
the [Crypto 2009 rump
session](https://twitter.com/StefanoMTessaro/status/1366914461886386176?s=20)),
and some comments by [Ducas](https://twitter.com/DucasLeo/status/1366957088954601473?s=20) (who is an expert on lattices) make it sound like a known strategy that doesn't seem to work out (he pointed to the paper [here](https://arxiv.org/pdf/1003.5461.pdf) from 2010, and also [experimentally tested some of the claims](https://github.com/lducas/SchnorrGate), demonstrating a gap between Schnorr's claim and experimental evidence).
Of course, this does not mean that RSA is completely safe (see [Heninger's
comments on the
matter](https://twitter.com/matthew_d_green/status/1366961403161174021?s=20)),
but it does dampen much of the excitement.

As I am giving a talk to the Crypto student seminar on Friday, I leafed through
the paper before deciding that I would not be able to present it well (both due
to the short notice, and the fairly technical nature of the claimed result).
In doing this, I saw a lattice which is quite similar to one from [one of Ducas
and Pierrot's (DP)
papers](https://eprint.iacr.org/2018/146.pdf).
The lattice itself seems to be defined in my advisor's book from 2002, but I
can't seem to get access to a digital copy via institutional access.

In this post I will try to discuss both lattices, although I will focus on
DP's contruction (which I understand better).
Both are related to the complexity of factoring --- Schnorr's is designed such
that short vectors help *break* factoring, while DP's is designed such that
the ability to *break factoring* allows one to solve the ``Bounded Distance
Decoding'' problem (which is a typical hard problem on lattices).
In what follows, I will focus on the $$\ell_2$$ norm (although DP also has
results for the $$\ell_1$$ norm).
They are also both related to "logarithms of primes", although the Schnorr
construction uses the (standard) logarithm, while DP uses a *discrete*
logarithm.

# Lattice Preliminaries

A *lattice* is a discrete subgroup $$\mathcal{L}\subseteq \mathbb{Z}^n$$.
Equivalently, it is the image $$\mathbf{B}\mathbb{Z}^k$$ of the integers under
$$\mathbf{B}\in\mathbb{R}^{n\times k}$$ (a *basis* of the lattice).
Here, $$n$$ is the dimension, and $$k$$ is the rank.
Generally, one restricts to "full-rank" lattices, i.e. where $$k = n$$.

An important quantity associated with a lattice is the length of its *shortest
vector* $$\lambda_1(L) = \min_{\ell\in L\setminus\{0\}}\lVert \ell\rVert_2$$.
One can place balls of radii $$\lambda_1(L) / 2$$ centered at each lattice point
in a non-overlapping way.
This means that information theoretically, one can recover $$\ell$$ from a
perturbation $$\ell+e$$ for $$e$$ in one of the aforementioend balls.
The computational problem of doing this is known as the *bounded-distance
decoding problem*, and being able to solve it efficiently on input a random
lattice already suffices to solve the LWE problem (in fact, the LWE problem can
be interpreted as doing precisely this).

From a different perspective, lattices with "large $$\lambda_1(L)$$" can correct "larger amounts of noise", and
have applications in the construction of error-correcting codes.
There is a boring way to get a lattice with *arbitrarily large* $$\lambda_1(L)$$
though, which is based on the identity:

$$\lambda_1(\alpha L) = \alpha\lambda_1(L)$$

where $$\alpha L$$ is the same lattice $$L$$ with its basis scaled by $$\alpha$$.
For this reason, when trying to build "dense" lattices people often work with a
scale-invariant quantity related to $$\lambda_1(L)$$.
There are a variety you can choose[^centerdensity], but I will discuss the
normalized shortest vector:

$$\overline{\lambda}_1(L) = \frac{\lambda_1(L)}{\sqrt[n]{\det L}}$$

[^centerdensity]: Namely, you can also work with what I believe is called the
    *center density*, and defined as
    $$\frac{\mathsf{vol}(\mathcal{B}_n(\lambda_1(L)/2))}{\det L}$$
    Where $$\mathcal{B}_n(R)$$ is the $$n$$-dimensional ball of radius $$r$$.
    This is roughly the "$$n$$th power" of the quantity we work with, although
    there is an additional term related to the volume of the ball of radius 1.
    Note that this name might be slightly off, but I believe it is the one that
    *Conway and Sloane* refer to, but will not check for this post.

A fundamental result in the theory of lattices is *Minkowski's Theorem*, which
upper bounds $$\overline{\lambda}_1(L) \leq 2\mathsf{vol}(\mathcal{B}_n(1))^{-1/n}\sim \sqrt{2n/\pi e}$$.
There is a natural question --- how *close* can one get to this bound?
Read DP for a summary, but the main idea is that random lattices are within a
constant factor of the bound, and that there exist explicit constructions that
are similarly close (but that the decoding problem is open for).

Before DP, the highest-density lattice one could decode was the Barnes-Wall
lattice of density $$\Theta(\sqrt[4]{n})$$, with decoder by Micciancio-Nicolosi.
The lattice of DP can be decoded in $$\Theta(\sqrt{n}/\log n)$$ time, which is a
large improvement.

# The Lattices of Schnorr and DP
## Schnorr
Throughout this, let $$p_1,\dots, p_n$$ be distinct primes.
Schnorr introduces[^adleman] the lattice with basis:

[^adleman]: I have heard that this construction is initially due to Adleman (although
    there are slight differences between Schnorr's and Adleman's lattices).
    Unfortunately, people have indicated that Adleman's paper may not be easily
    available online.

$$B_{n, c} = \begin{pmatrix}
\sqrt{\ln p_1}& 0 & 0\\
0 & \vdots & 0\\
0 & 0 & \sqrt{\ln p_n}\\
N^c\ln p_1 & \dots & N^c \ln p_n
\end{pmatrix}$$

Here $$B_{N, c}\in\mathbb{R}^{(n+1)\times n}$$, and $$N = \prod_i p_i$$.
Schnorr then claims that solving the closest vector problem on this lattice with
target point $$t = e_{n+1}N^c\ln N$$ allows one to construct a triple $$(u, v,
|u - vN|)$$ that is $$p_n$$-smooth, meaning each member of the triple has all
prime divisors $$\leq p_n$$.

Given $$n+1$$ triples $$(u_i,v_i, |u_i-v_iN|)$$, one proceeds as follows.
By smoothness, we have that:

$$u_j = \prod_i p_i^{e_{i,j}},\qquad u_j-v_jN = \prod_i p_i^{e_{i,j}'}$$

It is simple to see that $$(u_j-v_jN)/u_j\equiv 1\bmod N$$, so $$\prod_i
p_i^{e_{i,j}-e_{i,j}'}\equiv 1\bmod N$$.
Any solution $$t_1,\dots,t_{n+1}\in\{0,1\}$$ which solves:

$$\sum_i t_i(e_{i,j}-e_{i,j}')\equiv 0\bmod 2$$

leads to a solution of $$X^2 - 1\equiv 0\bmod N$$, which (provided such $$X\neq
\pm 1\bmod N$$) yields two non-trivial factors of $$N$$.
In particular, one sets:

$$X = \prod{i = 1}^{n+1}p_i^{\frac{1}{2}\sum_{j = 1}^{n+1} t_j (e_{i,j} - e_{i,j}')}$$

One can see similarities between this and the [quadratic
sieve](https://en.wikipedia.org/wiki/Quadratic_sieve), although they do not
appear to be the same (and the quadratic sieve does not appear to have anything
to do with lattices).

Concretely, the above approach *does* work, and Leo Ducas has explicit
experiments showing parameter ranges that it works in.
But these parameters are much worse than Schnorr claims (from a cryptanalytic
point of view at least).

## DP

The lattice of DP is broadly similar, although it is now defined in terms of
*discrete* logarithms of primes, rather than traditional logarithms (there are
of course other differences as well).
It fixes $$n, m\in\mathbb{N}$$ such that $$(\mathbb{Z}/m\mathbb{Z})^*$$ is a
cyclic group[^cyclic], and sets:

$$\mathcal{F} = \{p\in\mathbb{N}\mid p\text{ prime}, p\nmid m, p \leq B\}$$

where $$B$$ is chosen such that $$|\mathcal{F}| = n$$.
Easy arguments show that $$B\sim n\log n$$.

[^cyclic]: This assumption is not strictly necessary, but relaxing it does not
    lead to better a construction.

Now, consider the group homomorphism:

$$\psi : \mathbb{Z}^n \to (\mathbb{Z}/m\mathbb{Z})^*,\qquad
(x_1,\dots,x_n)\mapsto \prod_{i = 1}^n p_i^{x_i}\bmod m$$

The kernel of this is a subgroup of $$\mathbb{Z}^n$$, and therefore a lattice.
One can write it as:

$$\mathcal{L} = \{(x_1,\dots,x_n) \in\mathbb{Z}^n \mid \prod_i p_i^{x_i}\equiv
1\bmod m\} = \{(x_1,\dots,x_n)\in\mathbb{Z}^n \mid \sum_{i = 1}^n x_i\log_g
p_i\equiv 0\bmod \varphi(m)\}$$

Here, $$\log_g p_i$$ is the *discrete log* of $$p_i$$ within the group
$$(\mathbb{Z}/m\mathbb{Z})^*$$.
Lattice cryptographers have a compact way to describe lattices of the above
form, namely via the notation:

$$\Lambda_q^\perp(g) = \{\vec x\in\mathbb{Z}^n\mid \langle \vec x,\vec g\rangle\equiv
0\bmod q\}$$

where $$q$$ need not be prime.
One might call this a "rank 1 $$q$$-ary lattice".
Such lattices satisfy a sort of universality property, in that one can
approximate a large class of lattices from lattices of this type[^approximate].

[^approximate]: This is discussed in [Shor and Eldar's Systematic Normal Form
    paper](https://arxiv.org/abs/1604.07800), which is morally similar to [Sloane
and Vaishampayan's investigations of projecting $$\mathbb{Z}^n$$ orthogonal to a
single vector](http://neilsloane.com/doc/FATS.pdf).
In fact, these kinds of results were behind [Eldar and Shor's attempt to put $$\mathsf{CVP}$$ in
$$\mathsf{BQP}$$ in 2016](https://arxiv.org/abs/1611.06999), which was itself
another exciting time. 

From this description, it is fairy standard to write down an explicit basis:

$$\begin{pmatrix}
1 & 0 & \dots &0& 0\\
0 & 1 & \dots &0& 0\\
\vdots & & \ddots & \vdots\\
0 & 0 & \dots & 1 & 0\\
-\frac{\log_g p_1}{\log_g p_n} & -\frac{\log_g p_2}{\log_g p_n} &\dots &
-\frac{\log_g p_{n-1} }{\log_g p_{n}} &
\varphi(m)
\end{pmatrix}$$

Besides the last 


Anyway, DP writes down an explicit (up to determining $$\mathcal{F}$$) lattice,
and shows that:

1. It is quite dense
2. One can efficiently decode it

The first point is not that surprising --- there are various results that
essentially say that $$\log_g t$$ is distributed "randomly" (where I mean this
in an informal way) within $$(\mathbb{Z}/m\mathbb{Z})^*$$.
As random lattices tend to have good density properties, it is perhaps not
surprising that a "random ish" lattice has good density (as actually
random[^random] lattices get within a *constant* factor of Minkowski's bound).
For this reason, I will focus on presenting the decoding algorithm.
The proof of correctness of the decoding algorithm itself can be seen as a proof
that $$\lambda_1(L)$$ is not too small.

[^random]: Here, while "random" is formal, you have to be careful what you
    mean by it. In particular, the space of all lattices can be described as a
    ``double coset'' space, and there is a natural probability distribution on
    this space known as the *Haar measure*. Many spaces unrelated to lattices
    have Haar measures though (such as any compact group), and I personally have 
    never gone down this rabbithole deep enough to have good pointers to learn more,
    or to have a good idea about what such a distribution explicitly looks like
    for lattices.

DP frame their results in terms of the $$\ell_1$$ norm, and then use the generic
inequality $$\lVert x\rVert_1 \leq \sqrt{n}\lVert x\rVert_2$$.
I will mimic the presentation in their paper, where they first discuss the
$$\ell_1$$ norm where the errors are *positive* (i.e. in $$\mathbb{N}^n$$),
before extending to arbitrary (discrete) errors.

Say that you are given a perturbed lattice vector $$t = x + e$$.
Then, one can compute:

$$\prod_{i = 1}^n p_i^{t_i} = \prod_{i = 1}^n p_i^{x_i}\prod_{i = 1}^n
p_i^{e_i}\equiv \prod_{i = 1}^n p_i^{e_i}\bmod m$$

This essentially the same as "syndrome decoding" if you are familiar with that.
One then has that $$\prod_{i = 1}^n p_i^{e_i} \leq B^{\sum_i e_i} = B^{r}$$

where $$r$$ is an $$\ell_1$$ upper bound on the size of decodable errors.
As long as this quantity does not "overflow" (i.e. as long as $$B^r < m$$),
everything works great.
This is because $$p_i^{e_i} < m$$ is a smooth number, so one can efficiently
factor it, and recover the $$e_i$$'s.
This leads to an $$\ell_1$$ bound of $$r < \frac{\log_2 m}{\log_2 B}$$.

For arbitrary errors, the above decomposition no longer works.
This is because one has that:

$$f = \prod_i p_i^{t_i} \equiv \prod_{i : e_i > 0}p_i^{e_i} \prod_{i : e_i <0}
p_i^{e_i}\equiv u/v\bmod m$$

Is now a "fraction".
One can recover this fraction by looking at the two-dimensional lattice $$x -
fy$$, where $$x, y\in\mathbb{Z}$$.
Under certain conditions, the shortest vector $$(x, y)$$ in this lattice is precisely
$$(u, v)$$.
So recovering $$(u, v)$$ can be reduced to a 2-dimensional SVP instance, which
is efficient to compute.
The "certain conditions" mean that the decoding bound is now $$r < \frac{\log_2(m/2)}{2\log_2(B)}$$, but this is essentially the same as before.

# Extensions?

If one examines the equations, $$B\sim n\log n$$ is required to get $$n$$
distinct primes in the construction, which by the prime number theorem requires
that $$B\sim n\log n$$.
The $$\ln n$$ factor away from optimal in this construction is implicitly a
$$\ln B$$ factor.
So there is a natural question --- can one generalize this construction to allow
for smaller $$B$$, to close this gap?

I personally suspect that the answer is no.
I thought about it a bit two years ago, by appealing to generalizations of the
notion of primality.
Essentially, if one was working in a system that had "many more primes" than one
would expect, it is possible that one could port over this argument (say if one
could take $$B\sim \ln n$$).
This may seem like nonsense at first, but recall that $$B$$ is a norm bound on
some objects, so while within a "1D" object like $$\mathbb{Z}$$ there are only
at most $$\ln n$$ elements of norm below $$\ln n$$, in a higher dimensional
object there can be many more.

Unfortunately, I was not able to make this strategy work.
The field of [abstract analytic number
theory](https://en.wikipedia.org/wiki/Abstract_analytic_number_theory) studies
such "generalized primes", and proves things like prime number theorems for
them.
I was unable to find a reasonable system that had dense enough primes (I
initially was hoping working over number fields would work, and only searched
for more abstract things after this failed).

Since then, I like to think of the construction more in terms of "de-randomizing" a construction that meets Minkowski's bounds by (implicitly) combining:

1. The fact that $$\log_g p_i$$ is "pseudorandom", so $$\Lambda_m^\perp((\log_g
   p_i))$$ is "pseudorandom" as well.
2. That rank 1 $$q$$-ary lattices approximate arbitrary lattices fairly
   well, so $$\Lambda_m^\perp((\log_g p_i))$$ is "fairly close" to a random
   general lattice
3. That random (general) lattices are within constant factors of Minkowski's
   bound

It would be interesting to explore this point of view further, and see if the
construction can be formalized in these terms.
In particular, it would be interesting to see where one loses a $$\ln n$$
factor in the above steps.
I would guess step 2, but I do not have a great understanding of how strong the
approximation one gets is quantitatively (it is of course stated in the papers I
discussed, but it is generally stated in terms of approximating the entries of
the Gram matrix of the lattice coordinate-wise.
I do not have a great intuition how this impacts things like $$\lambda_1(L)$$).

