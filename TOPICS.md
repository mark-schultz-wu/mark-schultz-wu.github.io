Ideas for posts:

1. Why choose representatives {-q/p, ..., q/p-1} for norm of elements on Z_q
   (talk about the Lee metric/quotient metrics more generally).

2. Develop analogy between CRT and Lagrange Interpolation
  * View Lagrange Interpolation as statement about Vandermonde matrix
  * Fourier-theoretic interpretation of CRT?

3. Shamir's Secret Sharing via Linear Algebra:
  * Talk about Vandermonde Matrix, recast in terms of linear algebra + fourier analysis.

4. General OTP
  * View "Unif(G) + X = Unif(G)" from a fourier-analytic perspective?
    - should be pointwise mult of characteristic functions or w/e. Simpler proof?

5. Lattice Bound Computations

6. Discrete CLT?
  * CLT can be viewed as stating \sum_i X_i has limiting distribution N(0, \sqrt{N}sigma^2) (not dividing by N)
  * Replace X_i w/ discrete r.v.'s - sum now has Z support. What distribution is it?

7. The hash h_{a, b}(x) = (<a, x> + b) mod p where a, x in F_{p^k}, <a, x> takes their base-p decomp then inner product.
  This is strongly-universal (Mitzenmacher p 403). How similar is this to gadget-decomp stuff in lattices?

8. MAJ-boosting (BPP) vs Existential-boosting (RP/coRP).

9. 3D Smoothing parameter visualization/general writeup
  * Also some Fourier analysis (smoothing parameter as f_hat(0) coefficient?)
  * Intuition of packing radius, smoothing parameter as "fuzzy" version of packing radius due to Gaussian concentration on shells in high dimensions.

10. Stegonography implementation. Needs cover message distribution, play two chat bots off of eachother. Should be able to queue up data to send (in one/both directions) and send undetectably.
