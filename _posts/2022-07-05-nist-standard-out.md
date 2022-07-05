---
layout: post
title: NIST-PQC Choices Out
---

Today, [NIST put out its preliminary choice of PQC algorithms to standardize](https://csrc.nist.gov/publications/detail/nistir/8413/final).
I wrote an introduction to lattice-based KEMs (for non-cryptographers) this last weekend in preparation for the report, where one implements an (aggressively unoptimized) variant of FrodoKEM.
FrodoKEM ended up not being standardized, but the "broad picture" of the construction should still be useful to understand Kyber.

Anyway, I am writing this small post to collect links to the three posts on a single page.

* [Part 1]({% post_url 2022-07-02-lattices-for-programmers-pt1 %}) discusses the base underlying hardness assumption (Learning with Errors) some.
* [Part 2]({% post_url 2022-07-02-lattices-for-programmers-pt2 %}) constructs a simple lattice-based (private-key) cryptosystem, and
* [Part 3]({% post_url 2022-07-02-lattices-for-programmers-pt3 %}) constructs a lattice-based (public-key) cryptosystem.

---
