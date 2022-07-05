---
layout: post
title: Public Key Lattice-based Encryption
---

This is a continuation on my series on a simple construction of (public-key) lattice-based encryption.
We're finally ready to construct public-key encryption!
Similarly to the private-key setting, we will first give a construction in the (simpler, but insecure) *noiseless* setting, before adapting the construction to handle noise.

# Noiseless (public-key) Lattice-based Encryption

For private-key encryption, our construction was easiest to understand in terms of the one-time pad.
For public-key encryption, things are instead easiest in terms of *Diffie-Hellman Key Exchange*.
Recall that, for a fixed element $$g\in \mathbb{F}_p$$, this works by

1. sampling $$r, s\gets \mathbb{F}_p$$ (technically $$\mathbb{Z}/(p-1)\mathbb{Z}$$, but ignore this detail),
2. exchanging $$g^r, g^s$$, and then
3. having each party compute $$(g^r)^s = g^{rs} = (g^s)^r$$.

Diffie-Hellman works by computing $$g^{rs}$$ in two ways, where eacy way involves the "secret key" ($$r$$ or $$s$$) and "public key" ($$g^r$$ or $$g^s$$).
For the (noiseless) LWE assumption, our secret key is $$\vec s$$, and public key is $$A\vec s$$.
Can we use this to "compute something in two ways" again?

The answer is yes!
In particular, we will compute $$\vec r^tA\vec s$$ in two ways, namely via $$\vec r^t(A\vec s)$$, and $$(\vec r^t A)\vec s$$.
This means that we need the noiseless LWE sample $$(A, A\vec s)$$, and the *transposed* sample $$(A^t, A^t\vec r)$$ (as $$(A^t\vec r)^t = \vec r^t A$$).
One of them we will store long-term (in the public key), while the other we will generate "fresh" (and store in the ciphertext).
Without further ado, we have the following (noiseless) public-key cryptosystem

    import random
    from utils import *

    class NoiselessPubKey:
        def __init__(self, n, q):
            self.n = n
            self.q = q
        def key_gen(self):
            A = sample_unif_matrix(self.n, self.q)
            self.sk = sample_unif_vector(self.n, self.q)
            As = matrix_vector_multiply(A, self.sk, self.q)
            self.pk = (A, As)

        def enc(self, m):
            (A, As) = self.pk
            # Sampling r
            r = sample_unif_vector(self.n, self.q)
            # Computing u := A^tr
            At = matrix_transpose(A)
            u = matrix_vector_multiply(At, r, self.q)
            # Computing r^t(As)
            rAs = vector_vector_inner_product(r, As, self.q)
            # Using r^t(As) as a random pad
            v = (rAs + m) % self.q
            return (u, v)

        def dec(self, ctxt):
            (u, v) = ctxt[0], ctxt[1]
            s = self.sk
            # Computing (r^tA)s
            rAs = vector_vector_inner_product(u, s, self.q)
            # Subtracting rAs from v
            m = (v - rAs) % q
            return m

So ciphertexts contain the term $$v = \vec r^tA\vec s + m$$, and the share $$(A^t, A^t\vec r)$$ needed to decrypt.
Actually, we can fully omit $$A^t$$ (as it already in the public key) for some small space savings.

While encryption/decryption are quite fast for this scheme (well under $$.10$$ seconds), it has a *significant* flaw, namely that it can only encrypt a *single* element from $$\mathbb{Z}/q\mathbb{Z}$$, which for $$q\approx 3000$$ is only like 12 bits.
An (unoptimized) way to fix this is to simply break your plaintext up into 12-bit long blocks, and encrypt multiple things.
This introduces quite a bit of redundency that one can optimize away (and this is precisely what FrodoKEM does compared to what I have presented), but I will defer details of this to later.
The main idea is to use the above template scheme, but with many of the matrices/vectors "resized" (i.e. with certain parameters tuned).

Well, one shouldn't use *exactly* the above scheme, as it isn't secure!
With that, we'll discuss how one can make it secure.

# Noisy (public-key) Lattice-based Encryption

We again switch every usage of $$(A, A\vec s)$$ to $$(A, A\vec s + \vec e)$$.
This is fairly mechanical (and simple to verify that you got right), but as this verification step is somewhat technical (write down a security proof!) I will omit it.

    import random
    from utils import *

    class LWEPubKey:
        def __init__(self, n, q, B):
            self.n = n
            self.q = q
            self.B = B
    
        def key_gen(self):
            A = sample_unif_matrix(self.n, self.q)
            self.sk = sample_bounded_vector(self.n, self.B)
            As = matrix_vector_multiply(A, self.sk, self.q)
            e = sample_bounded_vector(self.n, self.B)
            b = vector_vector_add(As, e, self.q)
            self.pk = (A, b)
    
        def enc(self, m):
            (A, b) = self.pk
            # Sampling r
            r = sample_bounded_vector(self.n, self.B)
            e_prime = sample_bounded_vector(self.n, self.B)
            # Computing u := A^tr + e'
            At = matrix_transpose(A)
            u = matrix_vector_multiply(At, r, self.q)
            u = vector_vector_add(u, e_prime, self.q)
            # Approximately computing r^t(As)
            rAs = vector_vector_inner_product(r, b, self.q)
            # Adding e_double_prime so it is a "LWE encryption"
            e_prime_prime = sample_bounded_vector(1, self.B)[0]
            v = rAs+ e_prime_prime % self.q
            # Adding an encoding of m to the "random pad"
            v = (v + (self.q//2)*m) % self.q
            return (u, v)
    
        def dec(self, ctxt):
            (u, v) = ctxt[0], ctxt[1]
            s = self.sk
            # Approximately (r^tA)s
            rAs = vector_vector_inner_product(u, s, self.q)
            # Subtracting rAs from v
            v = (v - rAs) % self.q
            # Decoding (q//2)*m + error_terms
            return round(v / (self.q//2)) % 2

One can compute that the scheme is correct provided that $$\lfloor (\langle sk, e'\rangle + \langle r, e\rangle + e'') / (q//2)\rceil =0$$.
If we choose $$sk, \vec r$$ as they were before (uniformly at random from $$\mathbb{Z}/q\mathbb{Z}$$, this will barely ever happen.
Instead, we use the known result that one loses no security if one samples the LWE secret from the same distribution as the LWE error to sample $$\vec{sk}, \vec r$$ with each coefficient at most $$B$$.
This yields correct encryption provided that $$2nB^2 + B < q/4$$.
In practice you'll get some cancellations between the random terms, but I won't bother giving a refined analysis --- the intention is to point out that one can still get correct encryption, although in the public key setting you have to correct more error than the private key setting.

Note that the noisy setting can encrypt even less useful data than the noiseless setting --- a single element of $$\{0,1\}$$ now, rather than $$\mathbb{Z}/q\mathbb{Z}$$.
Again, this is merely an issue with the unoptimized scheme, and one can easily optimize things to encrypt more data.

# Conclusion

With that, we've constructed public-key lattice-based encryption!
The construction is an (aggressively unoptimized) version of the FrodoKEM cryptosystem, a NIST PQC round 3 (finalist/alternate, I forget).
The main optimizations we are missing out on are

1. the aforementioned technique of encrypting more than just a single bit, and
2. a technique to compress the random matrix $$A$$ in the public key (which is $$n^2\log_2 q$$ bits large --- fairly big) to a short 128-bit seed, and
3. optimizing the choice of bounded error distribution to be a *concentrated* bounded error distribution (think a Gaussian), allowing one to get tighter bounds on the noise one must correct.

Everything else is essentially the same.
Note that even among NIST candidates, FrodoKEM is an intentionally "unoptimized" one, meaning that it intentionally uses weaker security assumptions (at some cost in potential optimizations).
The (main) optimization it misses out on is that of appealing to *algebraically structured lattices*.
I might write up a blog post on this topic, but it is fairly advanced in comparison, as it most naturally phrased in terms of modules over quotients of polynomial rings.
While this topics are very possible to concretely work with (i.e. they aren't "abstract nonsense"), it is decidedly more advanced than simple matrix multiplications and modular arithmetic.

---
