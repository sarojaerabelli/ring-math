//! vector module
use std::ops::{Add, Mul, AddAssign};
use crate::traits::Zero;
use crate::polynomial::Polynomial;

/// Structure representing a vector of polynomials in a generic ring T[x]/(X^N - 1)
///
/// # Attributes
/// * `ring_degree` - the degree N of the polynomial ring
/// * `length` - the length of the vector
/// * `polys` - the values of the vector
pub struct Vector<T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign> {
    pub ring_degree: usize,
    pub length: usize,
    pub polys: Vec<Polynomial<T>>
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign> Vector<T> {

    /// Return a new zero polynomial.
    ///
    /// # Arguments
    /// * `ring_degree` - Degree N of ring.
    ///
    /// # Output
    /// * a new instantiation of a Polynomial, equal to 0
    /// ```
    pub fn new(ring_degree: usize, length: usize) -> Vector<T> {
        let mut poly_vec: Vec<Polynomial::<T>> = Vec::new();
        for _ in 0..length {
            poly_vec.push(Polynomial::<T>::new(ring_degree))
        }

        Vector {ring_degree,
                length,
                polys: poly_vec}
    }

    /// Compute the dot product of two vectors.
    ///
    /// # Arguments
    /// * `other` - other vector
    ///
    /// # Output
    /// * a new instantiation of a Polynomial, which is the dot product of the two Vectors
    /// ```
    pub fn dot_product(&self, other: &Vector<T>) -> Polynomial<T> {
        if self.length != other.length {
            panic!("Vector lengths should be equal. {} != {}", self.length, other.length);
        }

        let mut dot_prod: Polynomial<T> = Polynomial::new(self.ring_degree);
        for i in 0..self.length {
            dot_prod = dot_prod.add(&self.polys[i].multiply(&other.polys[i]));
        }
        dot_prod
    }

}

/// Structure representing a matrix of polynomials in a generic ring T[x]/(X^N - 1)
///
/// # Attributes
/// * `ring_degree` - the degree N of the polynomial ring
/// * `num_rows` - the number of rows in the matrix
/// * `num_cols` - the number of columns in the matrix
/// * `polys` - the values of the vector
pub struct Matrix<T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign> {
    pub ring_degree: usize,
    pub num_rows: usize,
    pub num_cols: usize,
    pub cols: Vec<Vector<T>>
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign> Matrix<T> {

    /// Compute the matrix product with a vector on the left.
    ///
    /// # Arguments
    /// * `other` - other vector
    ///
    /// # Output
    /// * a new instantiation of a Vector, which is the product
    /// ```
    pub fn multiply_by_left_vector(&self, other: &Vector<T>) -> Vector<T> {
        if self.num_rows != other.length {
            panic!("Matrix row length should equal size of vector. {} != {}", self.num_rows, other.length);
        }

        let mut prod_vec: Vec<Polynomial<T>> = Vec::new();
        for i in 0..self.num_cols {
            prod_vec.push(other.dot_product(&self.cols[i]));
        }
        Vector {ring_degree: self.ring_degree,
                length: self.num_cols,
                polys: prod_vec}
    }

}

#[cfg(test)]
mod tests;