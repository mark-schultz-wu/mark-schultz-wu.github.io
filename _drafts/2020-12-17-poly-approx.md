---
layout: post
title: Bernstein-type Polynomial Approximations
---


I have recently seen a talk on the paper [Efficient Homomorphic Comparison
Methods with Optimal
Complexity](https://eprint.iacr.org/2019/1234), which I have found quite
interesting.
I tried generalizing their result for a bit and hit a snag.
Since then I have some slightly new ideas, so I decided to flesh them out here
(before giving up if they go nowhere).
At a minimum I think that this gives an interesting presentation of their
result.

As the title makes clear, the paper is about computing *comparison queries*
using fully homomorphic encryption (FHE).
Comparison queries are of the form:

$$\mathsf{comp}(a, b) = \begin{cases} 1 & a > b \\ 0 & a = b \\ -1 & a <
b\end{cases}$$

While this particular definition of them may not be canonical, intuitively
comparison queries capture computing the function $$(a,b)\mapsto a>b$$, and are
quite important for computing things like $$\max(a, b), \min(a, b)$$, or even
$$\mathsf{sign}(a)$$.
All of these can be easily be computed given a method to homomorphically compute
a "step function", which is essentially what the problem is.

In most homomorphic encryption schemes, computations are (implicitly) done over
some ring of positive characteristic, where all functions can be represented by
sufficiently high-degree polynomials.
In this sense, homomorphic computations of $$\{+, \times\}$$ suffices to compute
any function, and FHE gives you precisely this.
So the problem already has a naive solution.

This solution is quite bad thouogh --- in secure computation (both FHE and 
Multi-party Computation), linear operations are "cheap", while non-linear
operations are "expensive".
Reducing the computation of $$\mathsf{comp}(a, b)$$ to a extremely-high-degree
polynomial therefore leaves a *lot* to be desired.

How high of degree is it precisely?
It is rather simple to show a degree bound of $$\Theta(q)$$, and in particular
$$q/2\leq d < q$$.
The
[Schwartz-Zippel  lemma](https://en.wikipedia.org/wiki/Schwartz%E2%80%93Zippel_lemma)
implies that any polynomial which is zero on a subset $$S\subseteq
\mathbb{F}_q$$ has degree $$|S|\leq d$$.
Threshold functions are zero on half of $$\mathbb{F}_q$$, giving the lower
bound.
Moreover, it is well-known that any function $$\mathbb{F}_q\to\mathbb{F}_q$$ can
be computed by a polynomial of degree at most $$q$$.
It suffices to build a family of "polynomial delta functions" $$f_c :
\mathbb{F}_q\to\mathbb{F}_q$$ where:

$$f_c(x) = \begin{cases} 1 & x = c\\ 0 & x\neq c\end{cases}$$

Given these, we can compute any function $$F(x)$$ as $$F(x) =
\sum_{k\in\mathbb{F}_q}F(k)f_k(x)$$.
To show reduce the degree of this polynomial $$< q$$, repeatedly apply [Fermat's Little
Theorem](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem)).
Finally, one can write $$f_c(x) = \prod_{\substack{j\in\mathbb{F}_q\\j\neq c}}
(x-j)$$ as the family of polynomial "delta functions" (technically $$f_c(c)$$
may not be 1, but one can fix this with a scaling factor).

This is all to say that *exactly* computing a threshold function is within a
factor 2 of the "worst degree possible".
For this reason, most people instead try to use some *approximation* to the
threshold function.
Here one can do slightly better, but there are more [lower
bounds](https://arxiv.org/pdf/math/0604324.pdf) (although these are over
$$\mathbb{R}$$).
One can try to beat this bound by using approximations by ratios of polynomials
(so-called "rational functions"), or various transcedental functions, but these
involve operations which are difficult to compute using FHE.

Anyway, this has been enough cryptographic motivation, so now I will discuss
uniform approximation of $$C[0,1]$$ over $$\mathbb{R}$$.

# Uniform Approximation of $$C[0,1]$$

First, what is $$C[0,1]$$?
It is the space of all continuous functions $$[0,1]\to\mathbb{R}$$.
Note that $$\mathsf{sgn}(x)$$ itself is both:

1. Not of the form $$[0, 1]\to\mathbb{R}$$
2. Not continuous

These are both mild issues --- we can use a linear transformation to shift the
domain of $$\mathsf{sgn}(x)$$, and can make it continuous by modifying it on an
arbitrarily small interval $$(-\epsilon, \epsilon)$$ around the origin.

The idea behind *uniform approximation* is to approximate an arbitrary function
$$f\in C[0,1]$$ well "at all points" within $$[0,1]$$.
In particular, our notion of error will be:

$$\lVert f(x)-g(x)\rVert_{C[0,1]} = \max_{x\in[0,1]}|f(x) - g(x)|$$

This is an "$$\ell_\infty$$" notion of error, and is intuitively the right
notion --- we want $$g(x)$$ to approximate $$f(x)$$ well at *all points*.
Uniform approximation of $$C[0, 1]$$ by polynomials is a famous result within
mathematics --- the [Weierstrauss Approximation
Theorem](https://mathworld.wolfram.com/WeierstrassApproximationTheorem.html).
The proof of this is non-constructive though --- it shows that for any $$f\in
C[0,1]$$ and any error bound $$\epsilon$$, there exists *some* polynomial
which uniformly approximates $$f$$ to error $$\epsilon$$, but gives no way to
find it.

Some decades later, the probabilist Sergei Bernstein gave a constructive proof
of the Weierstrauss approximation theorem via the
[Bernstein Polynomials](https://en.wikipedia.org/wiki/Bernstein_polynomial) (see
additionally [this excellent
survey](http://mae.engr.ucdavis.edu/~farouki/bernstein.pdf)).
These are an explicit set of polynomials which one can use to uniformly
approximate $$C[0,1]$$.
The Bernstein polynomials are in particular of the form:

$$b_{k, n}(x) = \binom{n}{k}x^k(1-x)^{n-k},\quad n\in\mathbb{N}, k\in[n]$$

These should have a very familiar form --- in particular,
$$\Pr_{X\sim\mathsf{Binom}(x, n)}[X = k] = b_{k, n}(x)$$, so they are
essentially the probability mass function of the Binomial distribution.
From this one can immediately derive properties such as $$\sum_{k\in[n]}b_{k,
n}(x) = 1$$ (as the total probability must be 1).
After defining the Bernstein polynomials, one can define the following
*Bernstein Operator*, which for $$n\in\mathbb{N}$$ maps:

$$f \mapsto \sum_{k\in[n]}f(k/n)b_{n,k}(x)$$

This operator is the one that maps a function $$f\in C[0,1]$$ to the
polynomial which uniformly approximates it.
There is a generalization of Bernstein polynomials for uniform approximation of
$$C[0,\infty)$$ known as the
[Szasz-Mirakyan Operator](https://en.wikipedia.org/wiki/Sz%C3%A1sz%E2%80%93Mirakyan_operator).
This again has a probabliistic interpretation --- this time as the probability
mass function of the $$\mathsf{Pois}(nx)$$ distribution.
It is rather natural that the Poisson distribution appears, as it is known to be
the "limit" of the binomial distribution as the number of trials goes to
infinity.

# The Polynomial Approximation of Cheon et al.

Cheon et al. suggest the following polynomial approximation of
$$\mathsf{sgn}(x)$$.
They define the "base function":

$$f_n(x) = \sum_{k=0}^{n}\frac{1}{4^k}\binom{2k}{k}x(1-x^2)^k$$

They then look at the iterated function composition $$f^{(d)}_n(x)$$, and
discuss how well it approximates $$\mathsf{sgn}(x)$$ for $$n = O(1)$$ and $$d$$
large.
My interest in this scheme is by trying to view their choice of $$f_n(x)$$ as
some Bernstein-type operator applied to the $$\mathsf{sgn}(x)$$ function.
If this is the case, we could try to apply their approximation to other
functions $$C[0,1]$$, and see what happens.

Our goal is to view their function as a sum $$\sum_{k = 0}^n
\mathsf{sign}(\?)b_{n,k}(x)$$, where $$b_{n,k}(x)$$ are related to the
probability mass function of some distribution.
This is (unfortunately) rather difficult to do --- this is because:

$$\begin{aligned}
\sum_{k = 0}^n b_{n,k}(x) &= \sum_{k = 0}^n x(1-x^2)^k\\
&=x\sum_{k = 0}^n(1-x^2)^k\\
&=x\frac{1-(1-x^2)^{k+1}}{x^2}\\
&=\frac{1-(1-x^2)^{k+1}}{x}
\end{aligned}$$

This is to say that the basis functions "do not sum to 1" for all $$x$$, even if we look at
the limit as $$n\to\infty$$.
So they can't possibly be the probability mass function of some distribution
parametrized by $$x$$.

Of course, the issue could be that Cheon et al. consider polynomial
approximation on the space $$C[-1, 1]$$ rather than $$C[0,1]$$.
We could transer their construction to $$C[0,1]$$ and analyze it there --- maybe
then our approach would work (although the "not summing to 1" thing seems like
it wouldn't be fixed by rescaling inputs).
Note that the function $$\varphi(x) = 2x-1$$ maps $$\varphi([0,1]) = [-1,1]$$.
It follows that $$\tilde{f}_n(x) = f_n(\varphi(x))$$ is in $$C[0, 1]$$, and will
be what we examine.
Interestingly, Cheon et al. suggest looking at (essentially) this function as well (see remark
2 on page 16), as it has integer coefficients.

We have that:

$$
\begin{aligned}
\tilde{f}_n(x) &= \sum_{k = 0}^n \frac{1}{4^k}\binom{2k}{k}(2x-1)(1
-(2x-1)^2)^k\\
&= \sum_{k = 0}^n \frac{1}{4^k}\binom{2k}{k}(2x-1)(4x-4x^2)^k\\
&= \sum_{k = 0}^n \binom{2k}{k}(2x-1)x^k(1-x)^k\\
\end{aligned}
$$

If we consider the basis functions $$b_{n,k}(x) = (2x-1)x^k(1-x)^k$$, do we get
the probability density function of some distribution?
Unfortunately no --- when $$x = 1/4$$, this is negative for all $$n, k$$, so
cannot be a probability.

# Applying Cheon et al's Argument to $$|x|$$

One issue that arose when trying to make sense of $$f_n(x)$$ was that it was an
odd function.
The innocuous multiplicand of $$x$$ (rather than something like $$x^2$$) lead to
both of the above issues with our approach.
This motivates analyzing an even function closely related to
$$\mathsf{sgn}(x)$$.
The most obvious one is the absolute value:

$$|x| = x/\mathsf{sgn}(x) = x\mathsf{sgn}(x)$$

It is not obvious to me that efficient polynomial approximation of $$|x|$$ is
useful for FHE --- in particular, one can use it to simulate $$\mathsf{sgn}(x)$$
(and therefore things like $$\max(a,b)$$) by using division, which is expensive
for FHE.
This is fine, just that this is the disclaimer that likely all that lays ahead
is some odd mathematical calculations.

Cheon et al. arrive at their approximation $$f_n(x)$$ by stating three
properties they want their polynomial approximation to satisfy:

1. $$f(-x) = -f(x)$$
2. $$f(1) = 1, f(-1) = -1$$
3. $$f'(x) = c(1-x)^n(1+x)^n$$ for some constant $$c > 0$$.

The first two are simply origin symmetry and some boundary conditions, which are
natural enough.
Their third condition they motivate by stating that it increases the convexity
of $$f$$, which speeds convergence in their algorithm.

I'll take the above three conditions, modified in the "obvious" way to the case
of $$|x|$$.
Note that $$f'(x)$$ must be modified as well --- the derivative of an even
function must be an odd function.
I will therefore modify it to be $$f'(x) = cx(1-x)^n(1+x)^n$$.

It is an exercise in calculus to show that these conditions uniquely specify the
polynomial $$x^2(1-x^2)^n$$.
This polynomial *does* have a probabilistic interpretation, namely as the
probability mass function of a $$\mathsf{Geom}(x^2)$$.
Of course, this is still for the purposes of approximating functions within
$$C[-1,1]$$.
To make the comparison with Bernstein clearer, we chould pre-compose with
$$x\mapsto 2x-1$$, giving:

$$(2x-1)^2(1-(2x-1)^2)^n= (2x-1)^2(4x-4x^2)^n = 4^n(2x-1)^2(x-x^2)^n$$

This seems like clearly the wrong thing to do.
So how do we transport Cheon et al's construction to $$C[0,1]$$?
We could instead argue that they have already done that --- the function
$$x\mapsto x^2$$ maps $$[-1,1]\to [0,1]$$ in a way that "increases convexity"
(vaguely).
Their polynomial approximation method on $$C[0,1]$$ would then just be applying
the Bernstein construction to the $$\mathsf{Geom}(x)$$ random variables, which
is much cleaner (at least).

Is this a good (generalized) polynomial approximation method?
There are two indications that the answer is unclear:

1. I can't find anything in the literature about it
2. It's not even clear how to apply it to $$|x|$$

In particular, one would hope that we could map:

$$
\begin{aligned}
|x|&\mapsto \sum_{k = 0}^n |k/n| \Pr_{X\sim\mathsf{Geom}(x)}[X = k]\\
&= \sum_{k = 0}^n |k/n| x(1-x)^k\\
\end{aligned}
$$

One could maybe argue that there should be $$x^2$$ in this instead of $$x$$ (odd
terms in the approximation of an even function do not seem useful), but
this doesn't change that this quantity is (in the limit $$n\to\infty$$)
$$\mathbb{E}_{X\sim\mathsf{Geom}(x^{2})}[x]/n = \frac{1-x^2}{x^2n}$$.
This is to say that the naive way to apply our hypothetical polynomial
approximation scheme gives an answer which is *far* from the polynomail
$$x^2(1-x^2)^m$$ that Cheon et al's "3 properties" method gives us.

# Conclusion

It still seems quite interesting to generalize Cheon's construction (with the
goal of "polynomial approximation using $$\{+,\times,\circ\}$$"), but approaches
which try to view their construction as a Bernstein-type construction seem to
run into (many) issues.

1. The computations just don't work out (although this could maybe be fixed with
   some additional work)
2. The polynomial approximation scheme does not seem to show up in the
   literature, so one would have to develop it generically before trying to
   apply it to FHE
3. "Convexity issues"

This last part deserves elaborating.
Recall that Cheon et al's idea was to find some fixed "base" polynomial, and
then *compose* it.
This is why they cared about increasing convexity (so in each "iteration",
points get "pushed" in the right way).
It is not clear how a suitably-developed Bernstein-type approximation scheme
could be used for such a purpose.
You would at least need not only polynomial approximation of $$f(x)$$, but also
of its derivatives.
One could try to look for polynomial approximation of $$f(x)$$ in the
$$C^2[0,1]$$ norm:

$$\lVert f(x)\rVert_{C^2[0,1]} = \max_{x\in[0,1]}|f(x)| +
\max_{x\in[0,1]}|f'(x)| + \max_{x\in[0,1]}|f''(x)|$$

This seems like it would be in the right direction at least, but it is not clear
if this is better than viewing their construction as an iterative algorithm.
In particular, they mention that prior art was to apply Newton's method to the
polynomial $$f(x) = 1 - 1/x^2$$, which has roots at $$\pm 1$$.
This has the benefit that the update step $$x_{n+1} = g(x_n)$$ is computed by
$$g(x)$$ a polynomial, which is *much* more efficient for FHE.
The polynomial is in fact a special case of their polynomials (in particular
$$f_1(x)$$).
This suggests a different generalization, namely by understanding polynomials
for which Newton iteration is a polynomial function (which happens if
  $$r'(x)\mid r(x)$$).
This method is known as Newton-Schulz iteration, and apparently it has a
[generalization via Pade
approximation](https://nhigham.com/2020/12/15/what-is-the-matrix-sign-function/)
which I might look into soon (say using [this
reference](https://www.maths.manchester.ac.uk/~higham/fm/OT104HighamChapter5.pdf)),
or [this](http://eprints.ma.man.ac.uk/1451/1/covered/MIMS_ep2010_18.pdf).
Pade approximants $$\ell/0$$ are just truncated Taylor series, so their function
is just a truncated taylor series.
The "interesting" part is likely that it is the truncated taylor series for
$$\mathsf{sign}(x) = x/(x^2)^{1/2} = x/(1-(1-x^2))^{1/2}$$.
