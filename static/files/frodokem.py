"""
An implementation of a FrodoKEM-like Scheme in Python, where the priority is a
*succinct* implementation.
To this end, I will use numpy for matrix multiplication.
One should of course never do this in a non-toy scheme.
"""

import numpy as np
from typing import Tuple


class FrodoKEM:
    def __init__(self, n: int, q: int, error_dist: Tuple[int, int], bar_m: int, bar_n: int):
        self.n, self.q, self.bar_n, self.bar_m = n, q, bar_n, bar_m
        self.error_dist = error_dist
        self.uniform_dist = (-q//2, q//2)
        self.rng = np.random.default_rng()
        # Modular reduction to the interval [-q/2, q/2) rather than [0, q)
        self.reduce = np.vectorize(lambda x: (x % q) - q//2)

    def sample_error(self, n: int, m: int, distribution_support: Tuple[int, int]):
        """
        Samples an n x m matrix with entries that are i.i.d. uniform on an
        interval.

        inputs:
          n : the number of rows one wants to sample,
          m : the number of columns one wants to sample,
          distribution_support : a tuple (low, high). We sample from a uniform
              distribution on the integers [low, ..., high).

        output:
          a n x m random matrix with i.i.d. entries uniformly distributed on
              distribution_support.
        """
        (low, high) = distribution_support
        rand_array = self.rng.integers(low, high, size= n * m)
        return np.resize(rand_array, (n, m))

    def key_gen(self):
        n, bar_n, q = self.n, self.bar_n, self.q
        A = self.sample_error(n, n, self.uniform_dist)
        S_transpose = self.sample_error(bar_n, n, self.error_dist)
        E = self.sample_error(n, bar_n, self.error_dist)
        B = A @ S_transpose.T + E
        public_key = (self.reduce(A), self.reduce(B))
        secret_key = self.reduce(S_transpose)
        return (public_key, secret_key)

    def encode(self, message: [bool]):
        m = np.array(message)
        m.reshape((self.B, self.bar_n*self.bar_m))
        pass


    def encrypt(self, public_key, message: [bool]):
        n, n_bar, m_bar, q = self.n, self.n_bar, self.m_bar, self.q
        (A, B) = public_key
        assert len(message)  == self.B * self.bar_n * self.bar_m
        S_prime = self.sample_error(m_bar, n, self.error_dist)
        E_prime = self.sample_error(m_bar, n, self.error_dist)
        E_double_prime = self.sample_error(m_bar, n_bar, self.error_dist)
        B_prime = S@A + E_prime
        V = S_prime @ B + E_double_prime + 
        return (B_prime, V)


if __name__ == "__main__":
    # Constants for the level-one paramter set of Frodo-640
    n = 640
    q = 2**15
    # Chosen to have same support as true distribution
    error_dist = (-12, 12)
    B = 2
    bar_m = 8
    bar_n = 8
    Frodo640 = FrodoKEM(n, q, error_dist, bar_m, bar_n)

