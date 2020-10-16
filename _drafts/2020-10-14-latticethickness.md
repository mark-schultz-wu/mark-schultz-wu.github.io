---
layout: post
title: "Translating from Quadratic Forms to Lattices"
---

There is a well-known equivalence between *quadratic forms* and *lattices*.
This essentially comes down to the ability to write any lattice $$L$$ as:

$$L = \mathbf{B}\mathbb{Z}^n$$

for $$\mathbf{B}\in\mathbb{R}^{k\times n}$$.
The *Gram matrix* of this is then $$\mathbf{B}\mathbf{B}^t$$, which defines the
quadratic form:

$$Q(\vec{x}) = \vec{x}\mathbf{B}\mathbf{B}^t\vec{x}^t$$

If one instead writes the lattice as $$L = \mathbb{Z}^n \mathbf{B}^t$$, you can
write $$Q(\vec{x}) = \lvert \vec x^t\mathbf{B}^t\rvert_2^2$$ as the quadratic
form.

This is covered in most treatments of lattices from a mathematical perspective,
as a major application of the theory of lattices is in classifying various kinds
of quadratic forms.
The is precisely the *opposite* viewpoint that a cryptographer or coding
theorist would take.
Anyway, sometimes one is given a result in the "wrong langauge", and must
"translate" it to the correct one.
This post will do precisely this, after motivating the problem that will be
translated quickly.

# Lattices that are Good for Covering

While lattices are very often used for constructing sphere packings, they can
also be used to construct a *sphere covering*.

> Let $$\mathcal{C}\subseteq \mathbb{R}^n$$. We say that $$\mathcal{C}$$ is an
> $$r$$-covering if:
>
> $$\mathcal{C} + \mathcal{B}(r)\supseteq \mathbb{R}^n$$
>
> Where $$+$$ is the sumset/Minkowski sum, and $$\mathcal{B}(r)$$ is the
> $$\ell_2$$-ball of dimension $$n$$ and radius $$r$$.
> We call the minimum $$r$$ such that $$\mathcal{C}$$ is an $$r$$-covering the
> *covering radius* of $$\mathcal{C}$$, written $$\rho(\mathcal{C})$$.

We will restrict ourselves to the case that $$\mathcal{C}$$ is a lattice.
Our definition of covering radius agrees with the standard definition of the
covering radius of a lattice in this case.
The covering radius is manifestly not scale-invariant (it is easy to show that
$$\rho(cL) = |c|\rho(L)$$).
For this reason, people often work with the *thickness* of a lattice instead.

> The thickness $$\Theta(L)$$ of a lattice $$L\subseteq \mathbb{R}^n$$ with fundamental region
> $$\mathcal{V}$$ is defined to be:
> $$\Theta(L) =
> \frac{\mathsf{vol}(\mathcal{B}(\rho(L)))}{\mathsf{vol}(\mathcal{V})}$$

The thickness should be seen as a covering version of the notion of packing
density (or, as more commonly appears within lattice cryptography, the closely
related notion of the Hermite constant).
The thickness can be used to define a scale-invariant expression somewhat
related to the covering radius, in particular the expression:

$$\frac{1}{n}\log_2 \Theta(L) = \frac{1}{n}\log_2(\mathsf{vol}(\mathcal{B}(1))/\mathsf{vol}(\mathcal{V})) + \log_2 \rho(L)$$


I'm interested in finding lattices where:

1. The covering radius is "asymptotically good", especially in the range $$n\geq
   200$$ (much higher than traditionally considered in the theory of lattices)

2. The lattice admits efficient computation of a relatively close lattice point
   to an arbitrary point in space

Chapter 2 of *Sphere Packings, Lattices, and Groups* describes a number of
lattices, and mentions that many common choices of coverings have a common
defect:

$$\lim_{n\to\infty}\frac{1}{n}\log_2\Theta(L) =
\lim_{n\to\infty}\frac{1}{n}\log_2\Theta(\mathbb{Z}^n) \approx 0.2546$$

The coverings they discuss are of the form $$\mathbb{Z}^n$$, $$A_n^*$$, and
$$A_{n}^*\oplus\bigoplus_{i = 1}^m \Lambda_{24}$$ (where the $$A_n^*$$ term is
included to allow the lattice to be any dimension).
Note that this asymptotic equivalence is fairly weak (it essentially states
that, up to the term making everything scale invariant, that all of the covering
radii $$\rho(L) = 2^{cn}$$ for the same constant $$c$$), so one can still likely
optimize in the choice of quantizer among the ones mentioned above.
But they then mention a construction which *asymptotically beats* the above,
which is what I want to look into in this blog post.

# Beating Direct Sum Constructions for Covering

Conway and Sloane state that there are a *number* of constructions which achieve
$$\lim_{n\to\infty} (1/n)\log_2\Theta(L) < 0.2546$$, and that a particular
construction is upper-bounded by 0.084.
All of these constructions can be viewed in terms of a particular construction
of Davenport's, which constructs an $$n$$-dimensional lattice out of an
$$O(1)$$-dimensional lattice in a way that is "good for covering".
In particular, Conway and Sloane state that Davenport's result is of the
following form:

> Let $$L$$ be a fixed $$k$$-dimensional lattice with generator matrix $$M$$,
> and let $$\ell\in\mathbb{R}$$.
> Define the $$km$$-dimensional lattice $$L'$$ with generator matrix:
>
> $$M' = \begin{pmatrix}
M & 0&0&\dots & M/\ell\\
0 & M & 0 & \dots & M/\ell\\
0 & 0 & M & \dots & M/\ell\\
\vdots & \vdots & \vdots & \ddots & \vdots\\
0 & 0 & 0 & \dots & M/\ell
\end{pmatrix}$$
>
> Then, for $$\ell$$ fixed (and large), and $$m\to\infty$$, we have that:
>
> $$\lim_{m\to\infty}\frac{1}{mk}\log_2\Theta(L') \leq \log_2\sqrt{2\pi
> eG(L')}$$

Here, I should mention three things quickly:

1. $$G(L)$$ is the *mean-square error* of $$L$$. This is (another) way of
   measuring how good $$L$$ is for covering-related problems (details will not
   be important for this post, but it is the variance of a uniformly
   distributed random variable on the fundamental domain of $$L$$).

2. Davenport's construction contains the direct sum of $$m$$ copies of $$L$$, so
   can be seen as a "small twist" of the aforementioned direct sum constructions
   (while being asymptotically more efficient).

3. Davenport's construction can easily be (approximately) decoded if $$L$$
   can[^approxdecode].

[^approxdecode]:
    First decode the "last block" of $$L'$$ (which is of the form $$M\vec
   x/\ell$$), then subtract this off from all other coordinates, and decode each
   block independently.
   The quantization error in the last block is bounded by $$\epsilon' = \rho(L)/\ell$$, and in each
   other block it should be bounded by $$\vec{\epsilon}_i = \rho(L) + \epsilon'$$.
   It follows that the norm of all the quantization error is at most $$\lvert (\epsilon',
   \vec{\epsilon})\rvert_2 = \sqrt{\rho(L)^2/\ell^2 + m \rho(L)^2(1 +
   1/\ell)^2}\approx \rho(L)(1 + 1/\ell)\sqrt{m}$$

It's not yet clear if this simplistic decoder will suffice [^potentialissue], but to understand
Davenport's construction (and what is known about it) better I should really
read the source material, and to do this I need to translate things from the
language of quadratic forms to lattices --- hence this post.

[^potentialissue]: In particular, I
    thought that direct sum constructions should give lattices with quantization
error bounded by $$\sqrt{n}\rho(L)$$ for any "base" lattice $$L$$ already, so
this does not seem much better). There may be better efficient decoders
though, which I expect to be easier to construct after understanding the
construction.

# Davenport's Construction in terms of Quadratic Forms

Davenport's construction is in the paper [The Covering of Space by
Spheres](https://link.springer.com/article/10.1007/BF02843724).
There are a number of papers of this title --- my impression from the exposition
in Conway and Sloane is that they are follow-ups to Davenport's construction by
choosing "base lattices" of smaller mean-square error.
Note that Davenport's construction is *already* parameteric with respect to the
choice of "base" lattice.
He even claims the proof goes through for $$\ell_p$$ versions of things (for
$$p\in(0,\infty)$$).
I might try to see how this changes certain quantities (such as constants) at
the end.

Davenport's construction starts with a positive definite quadratic form $$q$$ in
$$k$$ variable of determinant $$d(q)$$, and $$N\in\mathbb{N}$$ a positive
integer.
He then defines:

$$
Q = v_{k+1}^2+\dots +v_h^2 + q\left(\frac{v_1}{N},\dots, \frac{v_k}{N}\right) +\sum_{i = 1}^nq\left(u_1^{(i)} + \frac{v_1}{N},\dots, u_k^{(i)} + \frac{v_k}{N}\right)
$$

Here $$n = mk + h$$ where $$k\leq h <2k$$, so $$Q$$ is a form in $$n$$
variables.
Davenport then shows that:

1. $$d(Q) = (d(q))^{m+1}N^{-k}$$

2. $$\mu^2(Q) < (m+1)(\overline{\mu}(q))^2 + \frac{k}{4}$$

Here:

$$\mu^2(Q) = \max_{\vec{x}\in\mathbb{R}^n}\mu^2(Q;\vec{x}) =
\max_{\vec{x}\in\mathbb{R}^2}\min_{\vec{y}\in\mathbb{Z}^n}Q(\vec{i}_1 -
\vec{x}_1,\dots, \vec{i}_n - \vec{x}_n)$$
