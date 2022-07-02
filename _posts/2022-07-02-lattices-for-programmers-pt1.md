---
layout: post
title: Textbook RSA and LWE
---

This is a first post in a series of blog posts where I plan on building a
relatively simple lattice-based public-key encryption scheme.
This construction is not novel at all [^frodoKEM], but the intention is to

* simplify certain optimizations that aren't needed for security, and
* present things in straightforward language, in terms of simple mathematical
  concepts.

[^frodoKEM]: For experts, the intention is to present [FrodoKEM](https://frodokem.org/), without any of
    the various compression optimizations, and with a simpler noise distribution.

Throughout, I will first present (insecure) constructions (that are *exceedingly* simple), before describing how we make them secure.
These insecure constructions correspond to "noiseless learning with errors".
As we will see later in this particular post, the "noise" is central to the overall security of constructions.

Mathematically, this series of posts will be rather simple.
The main things required will be matrix multiplication, and modular arithmetic.
Lattice-based encryption can involve more complex topics (such as lattices, and modules over polynomial rings), but these are needed solely to investigate the security of optimized constructions.
Our construction is (intentionally) far from optimized, allowing us to ignore these complex topics.

In this post I'll first talk about other elementary approaches to teaching cryptography (namely Textbook RSA), before introducing the underlying hard problem we will use (the Learning with Errors, or LWE problem).
I'll finish by giving some informal (but formalizable) justifications for why the LWE problem is hard.
Given the practical (and elementary) approach I am taking though, this justification will necessarily be at a high level.

# Textbook RSA

If you're a programmer interested in learning about cryptography, there are some
decent resources available.
One in particular is a scheme that cryptographers call *Textbook RSA*.
Most websites you might try to learn about RSA from cover this scheme (see
[GeeksForGeeks](https://www.geeksforgeeks.org/rsa-algorithm-cryptography/),
[Wikipedia](https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Operation), or
[Simplilearn](https://www.simplilearn.com/tutorials/cryptography-tutorial/rsa-algorithm)).
Really, if you haven't formally studied cryptography (but have tinkered around
with it a little bit), "textbook RSA" is likely exactly what you think the RSA
cryptosystem is.

Explicitly:

1. The public-key is a product of two primes $$N = pq$$, along with some value $$e$$
   (typically hard-coded to either 3, or the better choice [^better] of $$2^{16} + 1 =
   65537$$).
2. The secret-key is the pair $$(p, q)$$, along with an algorithm to compute "$$e$$th roots $$\bmod
   N$$", namely the map $$c\mapsto c^{1/e}\bmod N$$.
3. Encryption maps $$m\mapsto m^e\bmod N$$, and
4. Decryption maps $$c\mapsto c^{1/e}\bmod N$$.

[^better]: Small choices of $$e$$ can [lead to attacks](https://en.wikipedia.org/wiki/Coppersmith%27s_attack#Low_public_exponent_attack) (if other mistakes are made as well).

It is straightforward to see that this scheme is *correct*.
This is because

$$m\stackrel{\mathsf{Enc}}{\mapsto}m^e\bmod
N\stackrel{\mathsf{Dec}}{\mapsto}(m^e)^{1/e}\bmod N \equiv m\bmod N.$$

In words, encryption raises things to the $$e$$th power, and decryption takes
$$e$$th roots.
Provided these notions are defined sensibly, they are inverses, and everything
works out.

One then shows a variety of things, such as
* if you know $$(p,q)$$, you can efficiently take $$e$$th roots, and
* if you don't know $$(p,q)$$, the "obvious" way to take $$e$$th roots is factoring
  $$N$$ first [^inequivalent].

[^inequivalent]: It is known that factoring *suffices* to take $$e$$th roots, but it is not known to be *necessary*, i.e. there may be faster approaches.
The above is (roughly) a simple presentation of the RSA cryptosystem in a
"textbook" form.
Standard presentations describe in detail how you actually take $$e$$th roots
given the factorization $$(p, q)$$, but this won't be needed in my series.

## Merits of Textbook RSA

Clearly, the above scheme is *very simple* to describe.
Mathematically, both encryption and decryption are barely a line each.
Key generation is similarly (conceptually) simple.
Moreover, the underlying hard problem can be presented as being "Factoring".
This isn't really true, but it's somewhat close to the truth, and this problem is (conceptually) simple to understand.

## Issues with Textbook RSA

There are two broad issues I'd like to highlight, namely *implementation complexity* and *security*.

### Implementation Complexity

One issue with the above (simple) description of RSA is that it hides a good deal of complexity.
Standard implementations of RSA require the following algorithms be implemented

1. the extended GCD algorithm, and
2. primality testing.

Neither of these are that bad to do (having beginning programmers do them can be a good exercise).
For example, the following code snippet is a perfectly reasonable approach at implementing primality testing, as well as a simple rejection sampling algorithm to sample random (large) primes.

    import random

    def isprime(n):
        if n % 2 == 0:
            return False
        else:
            i = 3
            while i**2 < n:
                if n % i == 0:
                    return False
                i += 2
            return True

    def sampleprime(n):
        while True:
            p = random.randint(1, 2**n)
            if isprime(p):
                return p

This leverages that Python's standard library contains native support for big (2048-bit) integer arithmetic, so we can ignore implementing all of that.
Yay!

The issue with this naive approach is that it is utterly infeasible.
As mentioned, RSA requires 2048-bit integers, or 1024-bit primes.
To verify that a number is prime, the above primality test requires (roughly) $$\sqrt{n}$$ operations, or $$2^{512}$$.
This is to say that the above (natural) primality testing algorithm that many students happily write down is secretely an exponential-time algorithm, and therefore cannot be used for any actual instantiation of RSA.

Therefore, to *actually* implement RSA you need to discuss fast primality testing algorithms (for example [Miller-Rabin](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)).
This brings a great deal of complexity into the story (and we even got to skip the big integer implementation!).

This is all to say that while RSA is conceptually simple, actually *running* RSA for parameters that are thought to be secure requires a great deal of complexity.

### Security

The second issue is that RSA (as presented above) is not secure.
Cryptography has a minimal acceptable notion of security known as [Indistinguishability under Chosen-Plaintext Attack](https://en.wikipedia.org/wiki/Ciphertext_indistinguishability#Indistinguishability_under_chosen-plaintext_attack_(IND-CPA)).
Among other things, this implies that you cannot tell if two ciphertexts are encryptions of the same message.
But for the above "textbook RSA", if you encrypt the same message twice, you get back the same ciphertext (encryption is *deterministic*).
This is typically fixed by introducing a *padding scheme*, but these are both complex, and easy to get wrong (there have been *many* attacks in practice on improper RSA padding).

So Textbook RSA is simple to describe, but moderately complex to implement in a way that is reasonably efficient for parameters people actually use, and quite complex to implement in a way that is known to be secure.
RSA also has the issue that (even when properly padded) it is insecure against quantum computers, so once those develop further it'll be insecure in all settings.

## The Solution: Textbook LWE

While in this blog series I will present a simplified variant of lattice-based encryption (you might call it "Textbook LWE"), the simplifications will mean that the scheme is *less efficient*, rather than *less secure*.
While it will be less efficient, it'll still be *plenty* efficient compared to things like Textbook RSA --- for "real parameters" (actually *larger* parameters than those used in practice, as I'm somewhat lazy), all operations take less than a second.
There will be inefficiences to the scheme (in particular, the size of the ciphertexts will be poor), but these are things that can be easily fixed if one cares.

I will actually present two constructions of each algorithm under discussion, which I will call the *noiseless* and *noisy* constructions.
The noiseless construction will always be conceptually simpler, and capture "the essence" of what is happening (while being insecure).
The noisy construction will be secure, albient somewhat more complex than the noiseless construction.

With that, we'll discuss the underlying hard problem the constructions will be based on (akin to factoring for RSA, or more properly taking modular roots for RSA).

# The Learning with Errors Assumption

Similarly to the constructions I describe, I'll go over noiseless and noisy versions of the Learning with Errors Assumption.
Both are most simply phrased in terms of modular matrix arithmetic.
Throughout, all operations are done modulo $$q$$ for some (not necessairly prime) number $$q\in\mathbb{N}$$.
This is true even for "implicit" operations, for example the additions and multiplications that occur as part of a matrix-vector product $$A\vec v$$.

## Noiseless

> Let $$n, q\in\mathbb{N}$$ be integers. Let $$\vec s\gets (\mathbb{Z}/q\mathbb{Z})^n$$ be a uniformly random (secret) vector. Then, the Noiseless Learning with Errors assumption is that, for $$A\gets (\mathbb{Z}/q\mathbb{Z})^{n\times n}$$ a uniformly random (public) matrix, then $$(A, A\vec s)$$ is hard to distinguish from uniformly random.

So, the noiseless Learning with Errors assumption is about the pseudo-randomness of matrix-vector products $$A\vec s$$.
Without wasting too many words on this assumption, I'll simply say that standard linear-algebraic techniques (for example Gaussian elimination) suffice to show that this is insecure.
In particular,given $$(A, A\vec s)$$, it is straightforward to recover $$\vec s$$.
This is what we call in cryptography "bad".

## Noisy

One way to attempt to stop this attack is to introduce noise (we will discuss later how Gaussian elimination behaves with respect to noise).

> Let $$n, q\in\mathbb{N}$$ be integers, and let $$\chi$$ be a distribution on "small" integers modulo $$q$$. Let $$\vec s\gets (\mathbb{Z}/q\mathbb{Z})^n$$ be a uniformly random (secret) vector. Then, the Learning with Errors assumption is that, for $$A\gets (\mathbb{Z}/q\mathbb{Z})^{n\times n}$$ a uniformly random (public) matrix and $$\vec e\gets \chi^n$$, then $$(A, A\vec s +\vec e)$$ is hard to distinguish from uniformly random.

The hardness of this problem depends *heavily* on the underlying choice of parameters $$n, q, \chi$$ (and there are many variants of this problem I will not describe).
As mentioned before, if there is no noise (i.e. $$\chi$$ is concentrated on $$\{0\}$$) the problem is broken.
If $$\chi$$ is uniform on $$\mathbb{Z}/q\mathbb{Z}$$, is is also straightforward to show that the problem is "perfectly secure" (but unfortunately, it is not known to be useful for cryptographic constructions).
This leaves the intermediate case, where $$\chi$$ is a distribution on some subset of $$\mathbb{Z}/q\mathbb{Z}$$ (that is not solely $$\{0\}$$).
For even fairly small subsets (say $$[-10, 10]$$), this is thought to be secure.
If you actually want to get parameters that are plausibly secure, tools[^estimator] have been built by the community, but I won't bother formally checking the parameters I use are fine (they're larger than what everyone else uses, so it should be fine).

[^estimator]: https://lattice-estimator.readthedocs.io/en/latest/?badge=latest

# Reasons we think LWE is Hard

There are a *number* of perspectives on the LWE problem for why it is plausibly hard.
I'll describe a few below, where again given my elementary focus I will keep things

* at a high level, and
* non-technical.

This is to say that the rest of this section will be "informal but formalizable".

Before proceeding, I will say that the "real reason" why LWE is thought to be hard is because a lot of people have tried breaking it, and haven't had much success.
In particular, the best attacks take roughly time $$2^{cn}$$ for $$n\approx 0.28$$. This is better than RSA ($$2^{\sqrt[3]{n}}$$), and worse than elliptic-curve discrete logarithm ($$2^{n/2}$$).

It's an open secret in cryptography that we can't have "real reasons" why any particular problem is hard, as it is consistent with what is known in complexity theory that P = NP, and cryptography doesn't exist.
Of course, few cryptographers *believe* this, but we can't rule it out!
Still, there are a few interesting things you can say at a high-level about the hardness of LWE, so I'll briefly recount them.


## It is a Problem of Practical Interest

This isn't a reason to think that LWE is hard *per se*, but it is a reason that a certain community would be happy if LWE was easy.
From this perspective, you could say we have a win-win situation --- either cryptographers are happy (LWE is hard), or coding theorists are happy (LWE is easy).
Moreover, coding theorists have been trying to solve problems like LWE for a while (because it would be good for them), but have been so far unable to.
With that teaser, I'll go over the applications of LWE to coding theory.

Coding theory studies the construction of (among other things) *error-correcting codes*.
It is well-known in coding theory that

1. *Random constructions* often are "essentially the best" (in terms of amount of information transmitted, amount of errors that can be corrected, etc) among all possible constructions, and
2. Random *linear* constructions are often essentially as good as *random constructions*.

This is to say that when I talk about "random linear codes" below, this can be interpreted as "effectively optimal codes".

Anyway, LWE can be interpreted as a problem regarding the decodability of random linear codes.
In particular, the problem of going from $$(A, A\vec s +\vec e)$$ can be interpreted as being given a description of a random linear code ($$A$$), and a noisy encoding of a message ($$A\vec s + \vec e$$), and being asked to decode it to get back $$\vec s$$.
Note that this is slightly different than the statement of LWE that I used (which was simply that of *distinguishing* $$(A, A\vec s+\vec e)$$ from random --- a maybe easier task), but in most settings these problems are known to be equivalent.

In summary, if LWE is easy, coding theorists are able to efficiently decode a certain class of effectively optimal codes[^AWGN].
Note that this is different from other cryptographic problems --- factoring large numbers is not practically useful outside of attacking cryptographic constructions.

[^AWGN]: These codes are for the "Additive White Gaussian Noise Channel", roughly speaking codes for *analog* problems (like wireless communications) rather than *digital problems*.


## Gaussian Elimination

As we discussed before, Gaussian Elimination breaks noiseless LWE.
How does introducing noise change things?

For a (very) brief refresher on Gaussian elimination, you are given some system of equations

$$ax+by = e,\qquad a'x+b'y = e',$$

and must try to find some solution $$(x,y)$$ to both of them.
Gaussian Eliminatino proceeds by taking linear combinations of the equations to "cancel out" unknowns.
For example, multiplying the first equation by $$a'$$, the second by $$a$$, and subtracting leads to

$$(a'b-b'a)y = a'e-e'a.$$

Roughly speaking, this can let us "remove a dimension" from an LWE instance, i.e. convert a dimension $$n$$ LWE instance to a dimension $$n-1$$ LWE instance.
Removing this dimension comes at a cost though --- the error goes from $$|e|$$ to $$|a'e-e'a|$$, which is quite likely larger than $$2|e|$$.
This is to say that each dimension we double we (probably) lose a factor 2 in the error, so trying to solve LWE with Gaussian Elimination makes the error grow exponentially, so only works for *very* small error settings.

## A Warning Regarding NP-hardness

While there are many theoretical justifications for lattice-based encryption, I won't cover many of them (many are useful "asymptotically but not concretely").
Still, I'll explicitly discuss the issue of NP-hardness, as it is one that many people casually get wrong.

First, it is worth mentioning that NP-hardness is subtly the wrong notion for cryptography.
This is because it is a *worst-case* notion, while cryptography needs *average case* hardness.
For example, SAT is both

1. a worst-case hard problem (if you can solve it you can solve any NP problem!)
2. fairly easy to solve on "practical instances".

This is to say that for many distributions over SAT instances, SAT is actually *easy*, despite being worst-case hard.
This can happen for cryptographic problems as well --- for example it is usually rather straightforward to exhibit *some* prime factors of random integers (half of them are divisible by 2), which breaks the factoring problem (at least the standard way it is formulated).

Now, the LWE problem is *sort of* related to an NP-hard problem.
It's one of the things cryptographers get most excited about (called the "worst-case to average-case reduction").
The "sort of" there carries a lot of weight though, so lets talk about it.

There is a problem called $$\mathsf{SVP}_\gamma$$ (the Gap Shortest Vector Problem with approximation factor $$\gamma$$).
This is a decision problem, where given a matrix $$A$$ and number $$t$$, one has to decide if

1. $$\min_{\vec z \in \mathbb{Z}^n\setminus\{0\}} \lVert A\vec z\rVert < t$$, or
2. $$\min_{\vec z\in\mathbb{Z}^n\setminus\{0\}} \lVert A\vec s\rVert \geq \gamma t$$.

If $$t \leq \min_{\vec z\in \mathbb{Z}^n\setminus\{0\}} \lVert A\vec z\rVert < \gamma t$$, the algorithm can say whatever (both "true" and "false" count as correct behavior).
The idea is roughly as follows --- when $$\gamma = 1$$, the algorithm has to *perfectly* distinguish between the two cases, while for $$\gamma > 1$$, the algorithm only has to distinguish between instances that are "far apart" in a way that is measured by $$\gamma$$.
So the problem has gotten easier in some controlled way.

Anyway, $$\mathsf{SVP}_1$$ is (under some reasonable technical assumptions) NP-hard.
With some other technical details omitted, if you can solve LWE you can also solve $$\mathsf{SVP}_{\Omega(\sqrt{n})}$$.
Some people mis-interpret this, and say "lattice-based encryption is based on an NP-hard problem".
This is wrong though --- formally, $$\mathsf{SVP}_{\Omega(\sqrt{n})}$$ is known to be in NP and coNP, so (under even more technical assumptions[^PHcollapse]) *can't* be NP-hard.

[^PHcollapse]: Formally, it is not NP-hard unless the polynomial hierarchy collapses all the way.

There are other caveats about concretely applying the aforementioned connection to $$\mathsf{SVP}_\gamma$$ (that have to do with "tightness" of the reduction).
Roughly speaking, the issue is one of "hidden constants in $$O(\cdot)$$ notation".
For example, [one analysis](https://eprint.iacr.org/2016/360.pdf) of the aforementioned reduction is that if one can solve LWE in time $$T$$, then one can solve $$\mathsf{SVP}_{\Omega(\sqrt{n})}$$ in time $$\approx 2^{500}T$$.
It is possible that more efficient reductions are found, but the current ones are mostly (asymptotically) interesting.

## Quantum Hardness?

Lattice-based encryption (unlike RSA, or the elliptic-curve discrete logarithm problem) is thought to be hard for *quantum* computers as well.
I'm no quantum complexity theorist, so I'll briefly describe the justification I know for the hardness (although, similar to classical hardness, it really boils down to "many smart people have thought about it and failed to get fast algorithms").

Roughly speaking, many quantum algorithms that obtain *large* speed increases proceed by solving a very general problem(the [Hidden subgroup problem](https://en.wikipedia.org/wiki/Hidden_subgroup_problem)).
In particular, Factoring and the Discrete Logarithm problem can both be phrased as HSP instances and then solved in (quantum) polynomial time.
Can lattice-based encryption be phrased as an HSP instance?
The answer is (perhaps surprisingly) *yes*.

HSP instances are parameterzed by a choice of group $$G$$.
The HSP instances we know how to solve efficiently are mostly over *abelian* groups $$G$$.
Lattice-based encryption can be phrased as an HSP over a *non-abelian* group (the Dihedral group).
There are some reasons to think this abelian/non-abelian difference should be a structural "switching point" in the complexity of the problem (the HSP heavily relies on the quantum fourier transform, and fourier transforms are *much* nicer over abelian groups).

That all being said, there is nothing stopping someone from giving a quantum attack on LWE tomorrow, similarly to how there's nothing stopping someone from giving a linear-time algorithm for SAT tomorrow.
Such uncertainty is a core part of cryptography.

----
