//! polynomial module
use std::ops::{Add, Mul, AddAssign};
use crate::traits::Zero;

/// Structure representing a polynomial in a generic ring T[x]/(X^N - 1)
///
/// # Attributes
/// * `ring_degree` - the degree N of the polynomial ring
/// * `coeffs` - the coefficients of the polynomial
pub struct Polynomial<T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign> {
    pub ring_degree: usize,
    pub coeffs: Vec<T>
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign> Polynomial<T> {

    /// Return a new zero polynomial.
    ///
    /// # Arguments
    /// * `ring_degree` - Degree N of ring.
    ///
    /// # Output
    /// * a new instantiation of a Polynomial, equal to 0
    /// ```
    fn new(ring_degree: usize) -> Polynomial<T> {
        Polynomial {ring_degree,
                    coeffs: vec![T::zero(); ring_degree] }
    }

    /// Check length of coefficients.
    ///
    /// # Panics if length of coeffs does not match degree of ring.
    /// ```
    fn check_coeff_length(&self){
        if self.ring_degree != self.coeffs.len() {
            panic!("Ring degree should be equal to vector length. {} != {}", self.ring_degree, self.coeffs.len());
        }
    }

    /// Add a polynomial to another polynomial in the ring.
    ///
    /// # Arguments
    /// * `other` - other polynomial to add
    ///
    /// # Output
    /// * a new instantiation of a Polynomial, which is the sum of the two polynomials
    /// ```
    pub fn add(&self, other: &Polynomial<T>) -> Polynomial<T> {
        self.check_coeff_length();
        if self.ring_degree != other.ring_degree {
            panic!("Ring degrees should be equal. {} != {}", self.ring_degree, other.ring_degree);
        }

        let mut sum: Polynomial<T> = Polynomial::new(self.ring_degree);
        for i in 0..self.ring_degree {
            sum.coeffs[i] = self.coeffs[i] + other.coeffs[i];
        }
        sum
    }

    /// Multiply a polynomial to another polynomial in the ring.
    ///
    /// # Arguments
    /// * `other` - other polynomial to add
    ///
    /// # Output
    /// * a new instantiation of a Polynomial, which is the product of the two polynomials
    /// ```
    pub fn multiply(&self, other: &Polynomial<T>) -> Polynomial<T> {
        self.check_coeff_length();
        if self.ring_degree != other.ring_degree {
            panic!("Ring degrees should be equal. {} != {}", self.ring_degree, other.ring_degree);
        }

        let mut prod: Polynomial<T> = Polynomial::new(self.ring_degree);
        for i in 0..self.ring_degree {
            // Compute the x^i term in the product.
            for j in 0..=i {
                prod.coeffs[i] += self.coeffs[j] * other.coeffs[i - j];
            }
            // Compute the x^(N + i) term in the product.
            for j in (i+1)..self.ring_degree {
                // Set it to the x^i term since x^i = x^(N+i) in the polynomial ring.
                prod.coeffs[i] += self.coeffs[j] * other.coeffs[self.ring_degree + i - j];
            }
        }
        prod
    }

    /// Multiply a polynomial by x.
    ///
    /// # Output
    /// * a new instantiation of a Polynomial, which is the product of the polynomial with x
    /// ```
    pub fn multiply_by_x(&self) -> Polynomial<T> {
        self.check_coeff_length();
        let mut prod: Polynomial<T> = Polynomial::new(self.ring_degree);
        prod.coeffs[0] = self.coeffs[self.ring_degree - 1];
        for i in 1..self.ring_degree {
            prod.coeffs[i] = self.coeffs[i - 1]
        }
        prod
    }

}

#[cfg(test)]
mod tests;