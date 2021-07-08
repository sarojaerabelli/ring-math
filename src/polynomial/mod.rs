//! polynomial module
use std::ops::{Add, Mul, AddAssign};
use crate::traits::Zero;

/// Structure representing a polynomial in a generic ring T[x]/(X^N - 1)
///
/// # Attributes
/// * `ring_degree` - the degree N of the polynomial ring
/// * `coeffs` - the coefficients of the polynomial
pub struct Polynomial<T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign> {
    ring_degree: usize,
    coeffs: Vec<T>
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign> Polynomial<T> {
    /// Add a polynomial to another polynomial in the ring.
    ///
    /// # Arguments
    /// * `other` - other polynomial to add
    ///
    /// # Output
    /// * a new instantiation of a Polynomial, which is the sum of the two polynomials
    /// ```
    fn add(&self, other: &Polynomial<T>) -> Polynomial<T> {
        if self.ring_degree != other.ring_degree {
            panic!("Ring degrees should be equal. {} != {}", self.ring_degree, other.ring_degree);
        }

        let mut sum: Vec<T> = vec![T::zero(); self.ring_degree];
        for i in 0..self.ring_degree {
            sum[i] = self.coeffs[i] + other.coeffs[i];
        }
        Polynomial {ring_degree: self.ring_degree,
                    coeffs: sum}
    }

    /// Multiply a polynomial to another polynomial in the ring.
    ///
    /// # Arguments
    /// * `other` - other polynomial to add
    ///
    /// # Output
    /// * a new instantiation of a Polynomial, which is the product of the two polynomials
    /// ```
    fn multiply(&self, other: &Polynomial<T>) -> Polynomial<T> {
        if self.ring_degree != other.ring_degree {
            panic!("Ring degrees should be equal. {} != {}", self.ring_degree, other.ring_degree);
        }

        let mut prod: Vec<T> = vec![T::zero(); self.ring_degree];
        for i in 0..self.ring_degree {
            // Compute the x^i term in the product.
            for j in 0..=i {
                prod[i] += self.coeffs[j] * other.coeffs[i - j];
            }
            // Compute the x^(N + i) term in the product.
            for j in (i+1)..self.ring_degree {
                // Set it to the x^i term since x^i = x^(N+i) in the polynomial ring.
                prod[i] += self.coeffs[j] * other.coeffs[self.ring_degree + i - j];
            }
        }
        Polynomial {ring_degree: self.ring_degree,
                    coeffs: prod}
    }

}

#[cfg(test)]
mod tests;