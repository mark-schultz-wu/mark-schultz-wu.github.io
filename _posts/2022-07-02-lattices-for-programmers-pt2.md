---
layout: post
title: Private Key Lattice-based Encryption
---

In the previous post, we discussed

1. how "Textbook RSA" (as presented in tutorials) is often not able to be executed for parameters large enough to be plausibly secure,
2. what the LWE problem is, and
3. heuristic justifications for why it is hard.

We'll next construct an exceedingly simple (private key) lattice-based encryption scheme.
Throughout all of our implementions, we will need some basic functions, for example

* sampling uniformly random vectors/matrices,
* sampling vectors/matrices with bounded coordinates,
* matrix operations (multiplications, transpose, and addition).

These as straightforward to implement, but code is available [here](https://github.com/mark-schultz/lwe-for-programmers/blob/main/utils.py).
Anyway, in terms of these operations it is straightforward to implement private-key encryption.
We first go over the (conceptually) simpler noiseless case, before modifying the construction to handle noise.

# Noiseless (Private-key) Lattice-based Encryption

Given $$(A, A\vec s)$$, the (noiseless) LWE assumption states that this is indistinguishable from random.
While this assumption is *false*, we can still pretend it is true, and construct a cryptosystem that would be secure (were it true).
The main idea we need to leverage is that

1. under the noiseless LWE assumption, $$(A, A\vec s)$$ looks like $$(A, \vec u)$$ for uniform $$\vec u$$.
2. given a uniform vector $$\vec u$$, one can obtain secure encryption by an argument similar to the one-time pad.

This second point deserves some more discussion.
First, one might be tempted define $$\mathsf{Enc}_{\vec s}(\vec m) = (A, A\vec s \oplus \vec m)$$, where $$\oplus$$ is an operation such as XOR.
While this is a good instinct, it doesn't work out well here --- all of the arithmetic here is (standard) arithmetic modulo $$q$$, which XOR doesn't behave particularly well with respect to.

Another natural idea is to do the same thing, but with addition modulo $$q$$ replacing XOR.
This ends up working!
Namely, under the (false) Noiseless LWE assumption, one can show that $$\mathsf{Enc}_{\vec s}(\vec m) := (A, A\vec s+\vec m)$$ defines a secure encryption function.
Moreover, one can define decryption such that this function is correct --- it is straightforward to verify that $$\mathsf{Dec}_{\vec s}(A, \vec b)= \vec b - A\vec s$$ works (where all arithmetic is done modulo $$q$$).

This gives (after we have defined the aforementioned matrix/vector/sampling operations) the following simple cryptosystem.

    import random
    from utils import *

    class NoiselessPrivKey:
        def __init__(self, n, q):
            self.n = n
            self.q = q
        def key_gen(self):
            self.s = sample_unif_vector(self.n, self.q)

        def enc(self, m):
            # Sampling A
            A = sample_unif_matrix(n, self.q)
            # Computing b := As
            b = matrix_vector_multiply(A, self.s, self.q)
            # Adding m to As
            b = vector_vector_add(b, m, self.q)
            return (A, b)

        def dec(self, ctxt):
            (A, b) = ctxt[0], ctxt[1]
            # Recomputing As
            As = matrix_vector_multiply(A, self.s, self.q)
            # Recovering m = b - As
            for i in range(self.n):
                b[i] = (b[i] - As[i]) % q
            return b

Note that this code works for cryptographically large parameters (which is $$n\in [500,1000]$$ roughly and $$q\approx 2^{10}$$).
This is because lattice-based encryption is essentially just simple linear algebra on word-size integers [[^RING]]!
With this idea for how lattice-based encryption should work, we can re-introduce the noise, and fix the issues it brings about.

[^RING]: Here, "integers" is somewhat particular to FrodoKEM. Optimized schemes (based on structured lattices) such as Saber and Kyber instead do "linear algebra" with vectors/matrices of *polynomials*.

# Noisey (Private-key) Lattice-based Encryption

Introducing noise both doesn't impact the security of our construction at all, and changes everything.
This is because the natural noisy variant of our protocol (where we add $$\vec m$$ to the second component of $$(A, A\vec s +\vec e)$$, rather than $$(A, A\vec s)$$) is secure under the LWE assumption (similarly to how the prior version of our protocol can be shown to be secure under the noiseless LWE assumption).
The main difference is that the noiseless LWE assumption is clearly false, but the LWE assumption is thought to be true, i.e. we now have a secure protocol!

The issue we have to fix is correctness.
If we use the above modified encryption algorithm with our prior decryption algorithm, we get that

$$\mathsf{Dec}_{\vec s}(\mathsf{Enc}_{\vec s}(\vec m)) = \mathsf{Dec}_{\vec s}(A, A\vec s + \vec e +\vec m) = \vec m + \vec e\neq \vec m.$$

From this, we can see that the error being zero was key to the correctness of our prior (insecure) protocol.
Can we fix this correctness issue somehow?

The answer is (obviously) yes.
The solution is straightforward --- we need a way to encode the message $$\vec m$$ to tolerate additional noise $$\vec e$$.
While one can appeal to fancy techniques from coding theory, typically people just scale $$\vec m\mapsto (q/2)\vec m$$ (when the coordinates of $$\vec m\in\{0,1\}^n$$), or $$(q/p)\vec m$$ for $$\vec m\in\{0,\dots,p-1\}^n$$ more generally.

This is to say that encryption of the form

$$\mathsf{Enc}_{\vec s}(\vec m) = (A, A\vec s +\vec e + (q/2)\vec m)$$

suffices.
For decryption, we again subtract $$A\vec s$$ from the second component, but then need a way to map $$(q/2)\vec m+\vec e\mapsto \vec m$$.
To do this, we first divide by $$q/2$$ to get $$\vec m + \vec e / (q/2)$$, and then round to the nearest integer.
Provided the coordinates of $$\vec e / (q/2)$$ are small enough (concretely less than $$1 / 2$$, or equivalently the coordinates of $$\vec e$$ must have absolute value at most $$q/4$$), this will be correct, and we'll be done!

This leads to the (slightly different) code than before

    import random
    from utils import *

    class LWEPrivKey:
        def __init__(self, n, q, B):
            self.n = n
            self.q = q
            self.B = B
        def key_gen(self):
            self.s = sample_unif_vector(self.n, self.q)

        def enc(self, m):
            # Sampling A, e
            A = sample_unif_matrix(n, self.q)
            e = sample_bounded_vector(self.n, self.B)
            # Computing b := As + e
            b = matrix_vector_multiply(A, self.s, self.q)
            b = vector_vector_add(b, e, self.q)
            # Scaling m -> (q//2) m
            scaled_m = [(self.q//2) * m[i] % self.q for i in range(self.n)]
            # Adding (q//2)m to b = As + e
            b = vector_vector_add(b, scaled_m, self.q)
            return (A, b)

        def dec(self, ctxt):
            (A, b) = ctxt[0], ctxt[1]
            # Recomputing As
            As = matrix_vector_multiply(A, self.s, self.q)
            # Recovering scaled_m = b - As
            for i in range(self.n):
                b[i] = (b[i] - As[i]) % q
            # Scaling (q//2)m + e -> m
            m = [0 for _ in range(self.n)]
            for i in range(self.n):
                m[i] = round(b[i] / (self.q//2)) % 2
            return m

This gives us a complete implementation of (private-key) lattice-based encryption!
There is technically a security issues with this code (it uses pythons default RNG), but this is simple enough to fix.
Note that, even if one uses a secure RNG, you probably don't want to use this in production --- there are *much* more efficient options if one uses the optimizations we are (intentionally) omitting.

----
