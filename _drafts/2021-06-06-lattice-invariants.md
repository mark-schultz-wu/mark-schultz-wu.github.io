---
layout: post
title: Some Non-standard Invariants of Euclidean Lattices
---

# Introduction

There are a number of invariants [^invariants] of Euclidean lattices that are
well-known to lattice cryptographers.
Concretely, for a (full-rank, for simplicity) lattice $$L = \mathbf{B}\mathbb{Z}^n$$ for
$$\mathbf{B}\in\mathbb{R}^{n\times n}$$.
Then, the following are well-known (to cryptographers) invariants of the lattice:

[^invariants]: By an invariant of a lattice, I mean a quantity that is
    independent of the choice of basis of the lattice. One could likely also require
    the quantity is invariant under an orthogonal transformation of the lattice
    --- I will not examine this too closely.

1. The Voronoi region $$\mathcal{V} = \{x\in\mathbb{R}^n \mid \lVert x\rVert  <
   \lVert x - \ell\rVert \forall \ell\in L\setminus \{0\}\}$$

2. The volume of the Voronoi region, which is given by $$\det B$$

3. The shortest vector $$\lambda_1(L) = \min_{\ell\in L\setminus\{0\}} \lVert
   \ell\rVert$$

4. More generally, the successive minima $$\lambda_i(L) = \min_{r\in\mathbb{R}^+} \left(\dim\mathsf{span}_{\mathbb{R}}(\mathcal{B}_n(r)\cap L)\geq i\right)$$

5. The covering radius $$\rho(L) = \max_{x\in\mathbb{R}^n}\min_{\ell\in L}\lVert
   x - \ell\rVert_2$$

6. The smoothing parameter $$\eta_\epsilon(L) = \min_s \rho_{1/s}(L^*\setminus
   \{0\}) \leq \epsilon$$, where $$\rho_s(S) = \sum_{x\in S}\exp(-\pi \lVert
   x/s\rVert_2^2)$$

7. All of these parameters for the dual lattice $$L^* = \{x\in\mathbb{R}^n \mid
   \langle L, x\rangle\in\mathbb{Z}\}$$

One can give various relationships between all of these quantities.
For example, one has the simple equality

$$\det L = 1/\det L^*$$

There are more complex relationships though:

1. One can lower bound the smoothing parameter by a factor involving $$1 /
   \lambda_1(L^*)$$

2. Banaszczyk's famous *transference theorems* give that $$1 \leq
   \lambda_i(L)\lambda_{n-i}(L^*) \leq n$$ for all $$i\in[n]$$.

3. Minkowski's (Second) theorem, which states that $$\prod_{i = 1}^n
   \lambda_i(L)^{1/n} \leq \sqrt{n} (\det L)^{1/n}$$

4. Somewhat more simply, $$\det L = 1 / \det L^*$$

The point of this post is to discuss a set of lattice invariants that can be
seen as alternatives to the successive minima which have some desirable
properties, namely that the version of transference for them is *exact*.
These lattice invariants are mathematically well-known, but have only sparsely
appeared in the cryptographic literature recently [^regev].

[^regev]: Namely in a few works of Oded Regev (and others), specifically the
    [Reverse Minkowski theorem](https://arxiv.org/pdf/1611.05979.pdf) and the
[Nearly Optimal Embeddings of Flat Tori](https://drops.dagstuhl.de/opus/volltexte/2020/12646/).

I was drawn to these papers via the *Reverse Minkowski Theorem* paper, as many
number theorists seemed to get quite excited by it. These number theorists work
in an area called *Arkelov Theory*, which is perhaps not the most accessible
area if one solely cares about  Euclidean lattices.
Fortunately, I have found at least one down-to-earth exposition on the matter,
namely [Comparison of some Invariants of Euclidean Lattices](https://webusers.imj-prg.fr/~huayi.chen/Recherche/iccm2019_chen.pdf) by Huayi Chen.
Perhaps it would be best for the reader to simply read this paper, but I will
continue writing regardless.

The goal of this blog post will be to compare three natural sets of invariants,
namely:

* The successive minima

* The Gaussian mass

* The ``successive slopes''

These sets of invariants, while *a priori* quite different, all closely
approximate eachother. This close approximation is behind many of the
aformentioned relationships between lattice invariants, so is perhaps best to
explicitly point to, especially as the successive slopes are not particularly
well-known within cryptography.

# The Successive Slopes
