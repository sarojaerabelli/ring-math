//! polynomial module
use std::ops::Add;

/// Structure representing a polynomial in a generic ring T[x]/(X^N - 1)
///
/// # Attributes
/// * `ring_degree` - the degree N of the polynomial ring
/// * `coeffs` - the coefficients of the polynomial
pub struct Polynomial<T: Add<Output = T> + Copy> {
    ring_degree: usize,
    coeffs: Vec<T>
}

impl<T: Add<Output = T> + Copy> Polynomial<T> {
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

        let mut sum: Vec<T> = Vec::new();
        for i in 0..self.ring_degree {
            sum.push(self.coeffs[i] + other.coeffs[i]);
        }
        Polynomial {ring_degree: self.ring_degree,
                    coeffs: sum}
    }

}

#[cfg(test)]
mod tests;