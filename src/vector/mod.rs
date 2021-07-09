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

    /// Add a vector to another vector in the ring.
    ///
    /// # Arguments
    /// * `other` - other vector to add
    ///
    /// # Output
    /// * a new instantiation of a vector, which is the sum of the two vectors
    /// ```
    pub fn add(&self, other: &Vector<T>) -> Vector<T> {
        if self.length != other.length {
            panic!("Vector lengths should be equal. {} != {}", self.length, other.length);
        }

        let mut sum_polys: Vec<Polynomial<T>> = Vec::new();
        for i in 0..self.length {
            sum_polys.push(self.polys[i].add(&other.polys[i]));
        }
        Vector{ring_degree: self.ring_degree, length: self.length, polys: sum_polys}
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

    /// Add a matrix to another matrix in the ring.
    ///
    /// # Arguments
    /// * `other` - other matrix to add
    ///
    /// # Output
    /// * a new instantiation of a matrix, which is the sum of the two matrices
    /// ```
    pub fn add(&self, other: &Matrix<T>) -> Matrix<T> {
        if self.num_rows != other.num_rows {
            panic!("Row lengths should be equal. {} != {}", self.num_rows, other.num_rows);
        }
        if self.num_cols != other.num_cols {
            panic!("Column lengths should be equal. {} != {}", self.num_cols, other.num_cols);
        }

        let mut sum_vecs: Vec<Vector<T>> = Vec::new();
        for i in 0..self.num_cols {
            sum_vecs.push(self.cols[i].add(&other.cols[i]));
        }
        Matrix{ring_degree: self.ring_degree, num_rows: self.num_rows, num_cols: self.num_cols, cols: sum_vecs}
    }

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