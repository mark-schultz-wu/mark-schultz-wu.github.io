---
layout: post
title: Obstacles to the Calculus of Sensitivity
tags: differential-privacy
hidden: true
---

In my [previous post]({% post_url 2020-10-15-DPsensitivity %}), I discussed how
for an appropriate "discrete" form of the derivative $$\nabla$$, that we have
that:

$$\Delta_{\vec x, p}(f) = \lVert \nabla f(\vec x)\rVert_p$$

Where $$\Delta_{\vec x, p}(f)$$ is the local $$\ell_p$$ sensitivity of $$f$$ at
$$\vec x$$, and $$\lVert\cdot\rVert_p$$ is a suitably-defined operator norm.
At the end of that post I discussed the potential for a "sensitivity calculus".
I will further develop this concept in this post, with the aim of developing the
prerequisites for a programmatic sensitivity analysis.
Along the way, I'll run into a significant roadblock.
I'm writing all of this up to show *why* the idea I had at the end of the last
post ended up not working out.
As a result, this likely isn't very interesting to anyone besides myself, but it
means that I've tied up a loose end, and can move onto other things after being
*waaay* too interested in this in the last 24 hours.

# The rules of discrete differentiation

The idea here is to mimic the "differentiation" rules for standard calculus.
Some basic examples of these are the:

1. Sum rule
2. Product rule
3. Quotient rule
4. Chain rule

If we had equivalents of all of these, we would be in a rather good place.
I will first develop equivalents for these for the 1D discrete derivative,
before trying to generalize these to higher dimensions.

## The 1D case

Recall that the 1D discrete derivative (also known as the forward difference
operator) is defined as:

$$f'(n) = f(n+1) - f(n)$$

Its easy to see by inspection that this behaves well with respect to sums (and
is in fact linear).
As a result, I will ignore that case, and examine products.
One can easily compute:

$$
\begin{aligned}
(fg)'(n) &= f(n+1)g(n+1) - f(n)g(n)\\
&= (f'(n) + f(n))g(n+1) - f(n)g(n)\\
&= f'(n)g(n+1) + f(n)(g(n+1)-g(n))\\
&\stackrel{1}{=} f'(n)g(n+1) + f(n)g'(n)\\
&= f'(n)(g'(n) + g(n)) + f(n)g'(n)\\
&= f'(n)g(n) + f(n)g'(n) + f'(n)g'(n)
\end{aligned}
$$

We see in the above line marked 1 that we come *close* to the definition of the
standard product rule, but unfortunately fall short.
We still get a well-defined product rule though, so we move on to quotients.

$$
\begin{aligned}
(f/g)'(n) &= \frac{f(n+1)}{g(n+1)} - \frac{f(n)}{g(n)}\\
&= \frac{f(n+1)g(n) - f(n)g(n+1)}{g(n+1)g(n)}\\
&= \frac{f(n)g(n) + f'(n)g(n) - f(n)g(n) - f(n)g'(n)}{g(n+1)g(n)}\\
&= \frac{f'(n)g(n) - f(n)g'(n)}{g(n)^2 + g'(n)}
\end{aligned}
$$

We again get something *close* to what we would expect, but not the same.
We finally move onto the chain rule.
We note that this is slightly more subtle than before --- the domain and
codomain of $$f$$ and $$g$$ are not the same.
There are two possible solutions to this:

1. Restrict composition to functions $$f : \mathbb{N}^k\to\mathbb{N}^j$$
2. Do not make this restriction, and only restrict the "inner" function to have
   domain $$\mathbb{N}^k$$.

We can worry about this later though, and first try to simplify the expression
$$(f\circ g)'(n)$$.
One (again) gets a chain rule, but of a slightly different form:

$$
\begin{aligned}
(f\circ g)'(n) &= f(g(n+1)) - f(g(n))\\
&= f(g(n) + g'(n)) - (f\circ g)(n)
\end{aligned}
$$

Note that we require *very little* requirements on $$f$$, solely that its domain
matches up with $$g$$'s codomain.
We quickly mention that this doesn't mean suddenly that we use standard
"continuous" derivatives in our analysis (even if $$f : \mathbb{R}^n\to\mathbb{R}^m$$).
This is because the "top" of our analysis is the discrete Jacobian $$\nabla f$$,
and the rules we develop will propegate *discrete* derivatives "down" our
expression in particular.
So *even* if we are analyzing a continuous function, we will be analyzing it in
terms of the forward difference operator (i.e. discrete derivative).

## Higher Dimensions

What does the calculus of derivatives for a function $$f : \mathbb{N}^k\to\mathbb{R}$$ look
like?

Recall that we define:

$$(\partial/\partial x_i)f(\vec x) = f(\vec x + e_i) - f(\vec x)$$

It is routine to check that this leads to (except for the chain rule) precisely the same "rules", but in
terms of the partial derivatives rather than the standard derivatives.
I will repeat the analysis for the chain rule, as it is both simple, and
illustrates this point fairly well.

$$
\begin{aligned}
(\partial/\partial x_i)(f\circ g)(\vec x) &= (f\circ g)(\vec x + e_1) - (f\circ g)(\vec x)\\
&= f(g(\vec x + e_1)) - (f\circ g)(\vec x)\\
&= f(g(\vec x) + (\partial/\partial x_i)g(\vec x)) - (f\circ g)(\vec x)
\end{aligned}
$$

Given that this situation was simple, what about higher dimensions?
Once our codomain is $$\mathbb{R}^m$$, we seem to lose the product and
quotient rules.
This is simply because $$\mathbb{R}^m$$ does not have a great multiplication or
division operation to use in the first place.
The sum rule will still work for boring reasons, so what about the chain rule?

Let $$g : \mathbb{N}^k\to \mathbb{R}^n$$, and let $$f :
\mathbb{R}^n\to\mathbb{R}^m$$.
What can we say about $$\nabla (f\circ g)(\vec x)$$?
Well, at least the following:

$$
\begin{aligned}
\nabla (f\circ g)(\vec x) &= ((\partial/\partial x_1)(f\circ g)(\vec x),\dots,
(\partial/\partial x_k)(f\circ g)(\vec x))\\
&= ((f\circ g)(\vec x + e_1) - (f\circ g)(\vec x),\dots, (f\circ g)(\vec x + e_k) - (f\circ g)(\vec x))\\
&= ((f\circ g)(\vec x + e_1),\dots, (f\circ g)(\vec x + e_k)) -
\mathbf{1}\otimes (f\circ g)(\vec x)\\
&= (f(g(\vec x) + (\partial/\partial x_1)g(\vec x)),\dots, f(g(\vec x) +
(\partial/\partial x_k)g(\vec x))) - \mathbf{1}\otimes (f\circ g)(\vec x)
\end{aligned}
$$

Here, by $$\mathbf{1}\otimes (f\circ g)(\vec x)$$, I mean the tensor product of
$$\mathbf{1} = (1,1,\dots,1)\in\mathbb{R}^k$$ with $$(f\circ g)(\vec x)$$.
This is just the matrix $$((f\circ g)(\vec x),\dots, (f\circ g)(\vec x))$$, or
equivalently the "vector" of that shape where each entry is an $$m$$-dimensional
(column) vector.

This suggests that we *do* have another chain rule.
It may not be able to be written in a pretty way, but our initial goal was to
use computers to calculate this all anyway, so who cares?
We (in principle) have a chain rule, so can bravely go on to the next step.

# The brick wall you run into

At this point, I admit defeat.
This may seem premature, given that one *should* be able to write a computer
program to apply the above rules.
I haven't yet computed discrete derivatives of simple functions, but this also
seems relatively easy to do.

What isn't simple is the operator norm.
I was hoping that the sub-additivity and sub-multiplicativity combine with the
calculus we develop to allow us to get (potentially loose) upper bounds on the
various derivatives.
This intuition is right for *continuous* calculus, but fails in our discrete
setting.

Why is that?
Because of the chain rule.
The multivariable form of it is particularly nice --- specificially that:

$$\nabla (f\circ g)(\vec x) = (\nabla f)(g(\vec x))\cdot (\nabla g)(\vec x)$$

Note that this expresses the derivative of the *composition* in terms of the
(matrix) product of the derivatives (which are linear maps).
While this allows for compact notation, it *also* has an important property that
it behaves well with respect to sub-multiplicative norms (such as the operator
norm) --- in particular:

$$\lVert \nabla (f\circ g)(\vec x)\rVert_p \leq \lVert (\nabla f)(g(\vec
x))\rVert_p \lVert \nabla g(\vec x)\rVert_p$$

It is not clear how to upper bound the norms of some of the expressions I was
developing.
So of the two steps:

1. Rewrite derivative in terms of "simpler" derivatives automatically using a
   "differential calculus"
2. Upper bound this rewritten derivative using sub-additivity and
   sub-multiplicativity of the operator norm

The second step is the one which seems problematic.
In general, one would need a way to upper bound the operator norm of expressions
of the form:

$$\lVert (f(g(\vec x) + (\partial/\partial x_1)g(\vec x)),\dots, f(g(\vec x) +
(\partial/\partial x_k)g(\vec x)))\rVert_p$$

This seems intrinsically difficult.
If we unwrap what the operator norm means, this is bounding the expression:

$$
\begin{aligned}
\lVert \nabla (f\circ g)(\vec x)\rVert_p &= \sup\{\lVert \nabla (f\circ g)(\vec
x)e_i\mid i\in[|\mathcal{X}|]\rVert_p\\
&= \max_i\lVert (\partial/\partial x_i) (f\circ g)(\vec x)\rVert_p\\
&= \max_i \lVert f(g(\vec x) + (\partial/\partial x_i)g(\vec x)) - f(g(\vec x))\rVert_p
\end{aligned}
$$

One can use triangle inequality to separate this out, but I run into issues even
when trying to upper bound $$\lVert f(g(\vec x))\rVert_p$$.
The issue here (compared to the continuous case) is that this is a composition
of *potentially non-linear* functions.
It looks like with enough "continuous" data, there are areas of math that
attempt to get bounds like this for non-linear functions (for example
[this](https://arxiv.org/pdf/math/0506424v2.pdf)).
You could even do simpler things, and find a "suitably good" polynomial
approximation to $$f$$.
These techniques feel like they're starting to lead to incredibly loose bounds
though, so I'll decide to walk away from the brick wall now rather than continue
to hit my head against it until something breaks.
