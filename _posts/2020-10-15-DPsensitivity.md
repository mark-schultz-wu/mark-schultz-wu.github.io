---
layout: post
title: The Sensitivity in Differential Privacy as a Derivative Bound
---

Differential Privacy is an area which has seen wide interest
(both in academia and real-life) in recent years.
But what *is* differential privacy?
As I don't work in this area, I'm rather ill-suited for saying anything --- but
at least why does it have that name?
Frank McSherry (who, among others, won the [Godel
Prize](https://en.wikipedia.org/wiki/G%C3%B6del_Prize) for creating the area)
[states that it was one of several names
considered](https://crypto.stackexchange.com/questions/77324/what-does-the-term-differential-in-differential-privacy-mean), including others like "Marginal Privacy" or "Incremental Privacy".
He further states that he thinks the name "stuck" due to an analogy with
differential cryptanalysis.

This post is a relatively quick note on how "differential" objects show up
(implicitly) in differential privacy.
This is somewhat unsurprising, but formalizing this has been sufficiently weird
journey that it seemed like it should be shared with others.


# What is Differential Privacy?

To avoid this post dragging on too long, I'm going to answer "I'm not telling".
I'm going to do this *regardless if I really know*, so my response is actually
differentially private (ha).

For people who actually want to know, there are a variety of excellent technical
resources to learn.
In particular [The Algorithmic Foundations of Differential
Privacy](https://privacytools.seas.harvard.edu/files/privacytools/files/the_algorithmic_foundations_of_differential_privacy_0.pdf) by Dwork and Roth is great.
I'll defer to them for an introduction, and even adopt their notation.
Really, the entirety of this post is pointing out that a common parameter within
differential privacy can be interpreted in terms of a familar looking equation,
if you simply redefine what all of the symbols in this equation mean.
Of course, this is still interesting because these definitions are "natural" in
a certain sense, as you will see below.

# The Sensitivity as a Derivative Bound

As mentioned before, I will use the notation of Dwork and Roth's book.
Definition 3.1 and 3.8 define the $$\ell_1$$ and $$\ell_2$$ sensitivities.
Upon reading both of them, there is a straightforward generalization to
$$\ell_p$$ sensitivities, which I give below:

> The $$\ell_p$$ sensitivity of a function $$f :
> \mathbb{N}^{|\mathcal{X}|}\to\mathbb{R}^k$$ is:
>
> $$\Delta_p(f) = \max_{\vec{x}, \vec{y}\in\mathbb{N}^{|\mathcal{X}|} : \lVert \vec x - \vec y \rVert_1 = 1} \lVert f(\vec x) - f(\vec y)\rVert_p$$

Generally this is explained in a few steps.
First, it is stated (or already known) that $$\mathbb{N}^{|\mathcal{X}|}$$ is
the natural domain of an arbitrary database.
Intuitively, a *record* in a database is an element $$r\in\mathcal{X}$$, for some
finite set $$\mathcal{X}$$.
Then the *database* is a collection of records, where you count the multiplicity
of each record.
It therefore lives in $$\mathbb{N}^{|\mathcal{X}|}$$.
Next, one notes that the condition that $$\lVert \vec x-\vec y\rVert_1 = 1$$ can be
interpreted as "the databases $$\vec x$$ and $$\vec y$$ are neighboring", in that they
differ in a single record, and that record differs in value by 1.
Finally, the condition $$\lVert f(\vec x) - f(\vec y)\rVert_p$$ is interpreted as
quantifying how much $$f(\vec x) - f(\vec y)$$ can change on neighboring records.

Given language like "The $$\ell_p$$ sensitivity quantifies how much a function
$$f$$ can change on adjacent points", it seems entirely unsurprising that you
can characterize it in terms of a derivative-like concept.
But what *is* a derivative for a function with the domain $$\mathbb{N}^{|\mathcal{X}|}$$?

## Discrete Derivatives

First, I'll restrict to "1 dimensional" functions $$f :
\mathbb{N}\to\mathbb{R}$$, before generalizing to something more useful.

Recall that the (continuous) derivative is traditionally defined via the
*difference quotient*:

$$f'(x) = \lim_{h\to 0, h\neq 0}\frac{f(x+h) - f(x)}{h}$$

If we want to restrict to discrete domains, we have to make sure that $$x+h$$ is
discrete (as it must be in the domain of $$f$$).
In our setting, this means that $$h\in\mathbb{N}$$.
As we need $$h\neq 0$$, this itself really means that we need $$h = 1$$, so can
define:

$$f'(x) = \frac{f(x+1) - f(x)}{1} = f(x+1) - f(x)$$

This definition is fairly standard (known as the [forward difference
operator](https://en.wikipedia.org/wiki/Finite_difference)), and denoted
$$\Delta(f)$$.
As $$\Delta$$ is *already* a super overloaded symbol in this post, I will
instead refer to it as $$f'(x)$$.

Note that the forward difference is "truly a derivative".
In particular, there is a general notion of a derivative (called a
[derivation](https://en.wikipedia.org/wiki/Derivation_(differential_algebra)))
of operators that "behave like the calculus derivative".
These are operators $$D$$ that:

1. Are linear maps
2. Satisfy the "product rule":
$$
D(fg) = fD(g) + D(f)g
$$

Clearly the 1-dimensional continuous derivative satisfies these.
The forward difference does as well.

How do we generalize (continuous) derivatives to higher dimensions?
The way that most people learn is the [partial
derivative](https://en.wikipedia.org/wiki/Partial_derivative#Formal_definition):

> Let $$f : \mathbb{R}^n\to \mathbb{R}$$.
> The *partial derivative* of $$f$$ with respect to the variable $$x_i$$ is defined
as:
>
> $$\frac{\partial}{\partial x_i}f(\vec{x}) = \lim_{h\to 0, h\neq 0}
> \frac{f(\vec{x} +he_i) - f(\vec{x})}{h}$$

I will interpret the partial derivative as the 1D derivative of a particular
unary function.
The particular function is:

$$F_{\vec x, i}(t) = f(\vec x - \vec{x}_i + e_i + te_i)$$

One can check that in the continuous case this is such that $$F_{\vec x,
i}'(\vec x_i) = (\partial/\partial x_i)f(\vec x)$$.

When applied to the discrete derivative, one gets that:

> Let $$f : \mathbb{N}^n\to \mathbb{R}$$.
> For any $$\vec{x}\in\mathbb{N}^n$$ and $$i\in[n]$$, let $$F_{\vec{x}, i}(y) = f(\vec x - \vec{x}_ie_i + te_i)$$
> The *discrete partial derivative* of $$f$$ with respect to the variable
> $$x_i$$ is defined as:
>
> $$\begin{aligned}
 (\partial/\partial x_i)f(\vec{x}) &= F_{\vec{x}, i}'(\vec{x}_i)\\
&= f(\vec x + e_i) - f(\vec x)
 \end{aligned}$$

I'll quickly mention that (something close to this) [seems like it is used *some places*](https://en.wikipedia.org/wiki/Finite_difference#Multivariate_finite_differences).
The interpretation of the partial derivative as the "standard derivative" 

Due to the way I wrote it above, I suspect that for each fixed $$x_i$$ that
$$(\partial/\partial x_i)$$ is still a derivation, although it doesn't matter in
particular so I won't bother verifying it. For further motivation for why this
may be a sensible "discrete partial derivative", I'll point to a conceptually close definition of "$$\mathbb{F}_2$$-partial derivatives", which take a similar form (such as in [this
paper](http://web.cs.ucla.edu/~sherstov/pdf/directional.pdf), near theorem 1.5).

Of course, sensitivity is defined for functions $$f :
\mathbb{N}^{|\mathcal{X}|}\to\mathbb{R}^k$$, so we have one final step of
generalization to do.
Recall that the gradient is just a vector containing the partial derivatives in
each direction:

$$\nabla f(x) = ((\partial/\partial x_1)f(x),\dots, (\partial/\partial
x_n)f(x))$$

I will then define the "discrete gradiant" in the same way below:

$$\nabla f(x) = ((\partial/\partial x_1)f(x), \dots, (\partial/\partial x_n)f(x))$$

Before proceeding, we need to note that (when the codomain of $$f$$ is dimension
$$>1$$) that $$\nabla f(x)$$ is most naturally a *matrix*, whose $$i$$th column
is $$(\partial/\partial x_i)f(x)$$.
We will define the norm of a matrix to be its *operator norm*.
The general definition of this is included below:

> Let $$M : V\to W$$ be a linear operator, and let $$\lVert\cdot\rVert_V$$,
> $$\lVert\cdot\rVert_W$$ be norms on $$V$$ and $$W$$ respectively.
> Define the norm of $$M$$ by:
>
> $$\lVert M\rVert = \sup\{\lVert Mv\rVert_W \mid v\in V, \lVert v\rVert_V =
> 1\}$$

Of course, our domain is $$\mathbb{N}^{|\mathcal{X}|}$$.
What norm are we picking on it?
This choice is key to the entire construction, so I will discuss it in a little
more depth.

## Viewing $$\mathbb{N}^{|\mathcal{X}|}$$ as a normed space

The natural norm to put on $$\mathbb{N}^{|\mathcal{X}|}$$ is a norm induced by
viewing it as a subset of $$\mathbb{R}^{|\mathcal{X}|}$$.
This is precisely the choice I will make.
If this is so straightforward, why does this have its own separate section?

What is *less* straightforward is what the (closed) unit ball on
$$\mathbb{N}^{|\mathcal{X}|}$$ looks like.
Or actually, it's a little *too straightforward*.
This is clearly the intersection of the unit ball in $$\mathbb{R}^{|\mathcal{X}|}$$ with $$\mathbb{N}^{|\mathcal{X}|}$$.
What is a little odd is that, for any $$p\in (1,\infty)$$, it is *also* clear
that the only points in this ball are 0, along with the "standard basis vectors"
$$e_i$$.
Quickly notice that:

1. In the $$\ell_\infty$$ norm this isn't the case (for example, $$(1,1,\dots,
   1)$$ is in the ball).

2. This is an "easy" example of where the closed unit ball (things of norm
   $$\leq 1$$) is not the same as
   the closure of the open unit ball (things with norm $$<1$$).
   The open unit ball, in any $$\ell_p$$ norm, will just be $$\{0\}$$.

This is all to say that for $$p\in(1,\infty)$$, we have that:

$$\begin{aligned}
\{\vec x\in\mathbb{N}^{|\mathcal{X}|} \mid \lVert \vec x\rVert_p \leq 1\} &= \{0\} \cup \{\vec x\in\mathbb{N}^{|\mathcal{X}|} \mid \lVert \vec x\rVert_p = 1\}\\
&= \{0\}\cup \{e_i\mid i\in[|\mathcal{X}|]\}
\end{aligned}$$

The above implies that when we define the operator norm, the *particular* norm
we choose on the domain does not really matter (as long as it is not the
$$\ell_\infty$$ norm), as the elements of norm exactly 1 will always be
precisely the standard basis of the "overlying" vector space.
For this reason, we will write:

$$\begin{aligned}
\lVert M\rVert_p &= \sup\{\lVert Mv\rVert_p \mid
v\in\mathbb{N}^{|\mathcal{X}|}, \lVert v\rVert_p = 1\}\\
&= \sup\{\lVert Mv\rVert_p \mid v\in\{e_i\}, i\in[|\mathcal{X}|]\}
\end{aligned}$$

To denote the operator norm of a linear map $$M :
\mathbb{N}^{|\mathcal{X}|}\to\mathbb{R}^k$$.

We could be putting *any* $$\ell_p$$ norm on the domain while doing this, and
will end up with an equivalent definition.
We'll next show how this leads to a quite natural interpretation of the
$$\ell_p$$ sensitivity as the maximum of the operator norm of $$\nabla f(\vec{x})$$ over $$\vec{x}\in\mathbb{N}^{|\mathcal{X}|}$$.

# The $$\ell_p$$ sensitivity as a norm bound on the discrete gradient

Let $$f : \mathbb{N}^{|\mathcal{X}|} \to \mathbb{R}^k$$.
We will next show the following:

> $$\Delta(f)_p = \max_{\vec x\in\mathbb{N}^{|\mathcal{X}|}}\lVert \nabla f(\vec x)\rVert_p$$

The proof is rather straightforward.
By the characteriation of norm-1 elements of $$\mathbb{N}^{|\mathcal{X}|}$$, we
have that the operator norm will be:

$$\lVert \nabla f(\vec x)\rVert_p = \sup\{\lVert [\nabla f(\vec x)] e_i\rVert_p\mid i\in[|\mathcal{X}|]\}$$

This is clearly equivalent to $$\max_{e_i}\lVert \nabla [f(\vec x)] e_i\rVert_p$$.
Note that $$[\nabla f(\vec x)] e_i$$ is the $$i$$th column of $$\nabla f(\vec x)$$, which is precisely $$(\partial/\partial x_i)f(\vec x)$$.
It follows that:

$$\begin{aligned}
\max_{\vec x\in\mathbb{N}^{|\mathcal{X}|}}\lVert \nabla f(\vec x)\rVert_p &= \max_{\vec{x}\in\mathbb{N}^{|\mathcal{X}|}}\max_{i}\lVert (\partial/\partial x_i)f(\vec x)\rVert_p
&= \max_{\vec x\in \mathbb{N}^{|\mathcal{X}|}} \max_i \lVert f(\vec x + e_i) - f(\vec
x)\rVert_p
\end{aligned}$$

This is *nearly* the $$\ell_p$$ sensitivity, except we are maximizing over the
choice of $$\vec x$$ and $$\vec x + e_i$$, rather than $$\vec x$$ and $$\vec y$$
such that $$\lVert \vec x - \vec y\rVert_1 = 1$$.
But these can readily be seen to be equivalent --- as norm 1 elements in
$$\mathbb{N}^{|\mathcal{X}|}$$ are precisely standard basis vectors, we have
that $$\vec x, \vec y$$ that are adjacent are of the form $$\vec x = \vec y \pm
e_i$$ for some $$i$$, so maximizing over the former is equivalent to maximizing
over the later.

It follows that:

$$\Delta(f)_p = \max_{\vec x\in\mathbb{N}^{|\mathcal{X}|}}\lVert \nabla f(\vec x)\rVert_p$$
