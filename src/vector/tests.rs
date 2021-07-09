// Tests
use super::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use crate::utilities::{check_vecs_almost_equal, generate_random_float_polynomial_vector,
                       generate_random_complex_polynomial_vector, generate_random_modint32_polynomial_vector,
                       generate_random_modint64_polynomial_vector, generate_random_float_polynomial_matrix,
                       generate_random_complex_polynomial_matrix, generate_random_modint32_polynomial_matrix,
                       generate_random_modint64_polynomial_matrix};
use crate::ring::{Complex, ModInteger32, ModInteger64};
use crate::traits::{Abs, Zero, One};
use std::ops::{Sub, Div};
use std::cmp::{PartialOrd, PartialEq};
use std::fmt::Debug;

const MAX_TEST_DEGREE: usize = 16;
const F32_MULTIPLY_ERROR: f64 = 0.01;
const F64_ERROR: f64 = 0.0000000001;

fn test_distributive_matrix_float<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> + One<T> + PartialEq {

    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let num_rows = rng.gen_range(1..=MAX_TEST_DEGREE);
    let num_cols = rng.gen_range(1..=MAX_TEST_DEGREE);
    let vector: Vector<T> = generate_random_float_polynomial_vector::<T>(num_rows, ring_degree);
    let mat1: Matrix<T> = generate_random_float_polynomial_matrix::<T>(num_rows, num_cols, ring_degree);
    let mat2: Matrix<T> = generate_random_float_polynomial_matrix::<T>(num_rows, num_cols, ring_degree);

    // Test that v * (m1 + m2) = v * m1 + v * m2
    let sum1 = mat1.add(&mat2);
    let total1 = sum1.multiply_by_left_vector(&vector);

    let prod1 = mat1.multiply_by_left_vector(&vector);
    let prod2 = mat2.multiply_by_left_vector(&vector);
    let total2 = prod1.add(&prod2);

    assert_eq!(total1.ring_degree, total2.ring_degree);
    assert_eq!(total1.length, total2.length);
    for i in 0..total1.length {
        assert!(check_vecs_almost_equal(&total1.polys[i].coeffs, &total2.polys[i].coeffs, error));
    }
}

#[test]
fn test_distributive_matrix_float_f32() {
    test_distributive_matrix_float::<f32>(F32_MULTIPLY_ERROR);
}

#[test]
fn test_distributive_matrix_float_f64() {
    test_distributive_matrix_float::<f64>(F64_ERROR);
}

fn test_distributive_matrix_complex<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> + One<T> + PartialEq {

    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let num_rows = rng.gen_range(1..=MAX_TEST_DEGREE);
    let num_cols = rng.gen_range(1..=MAX_TEST_DEGREE);
    let vector: Vector<Complex<T>> = generate_random_complex_polynomial_vector::<T>(num_rows, ring_degree);
    let mat1: Matrix<Complex<T>> = generate_random_complex_polynomial_matrix::<T>(num_rows, num_cols, ring_degree);
    let mat2: Matrix<Complex<T>> = generate_random_complex_polynomial_matrix::<T>(num_rows, num_cols, ring_degree);

    // Test that v * (m1 + m2) = v * m1 + v * m2
    let sum1 = mat1.add(&mat2);
    let total1 = sum1.multiply_by_left_vector(&vector);

    let prod1 = mat1.multiply_by_left_vector(&vector);
    let prod2 = mat2.multiply_by_left_vector(&vector);
    let total2 = prod1.add(&prod2);

    assert_eq!(total1.ring_degree, total2.ring_degree);
    assert_eq!(total1.length, total2.length);
    for i in 0..total1.length {
        assert!(check_vecs_almost_equal(&total1.polys[i].coeffs, &total2.polys[i].coeffs, error));
    }
}

#[test]
fn test_distributive_matrix_complex_f32() {
    test_distributive_matrix_complex::<f32>(F32_MULTIPLY_ERROR);
}

#[test]
fn test_distributive_matrix_complex_f64() {
    test_distributive_matrix_complex::<f64>(F64_ERROR);
}

#[test]
fn test_distributive_matrix_modint32() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let num_rows = rng.gen_range(1..=MAX_TEST_DEGREE);
    let num_cols = rng.gen_range(1..=MAX_TEST_DEGREE);
    let vector: Vector<ModInteger32> = generate_random_modint32_polynomial_vector(num_rows, ring_degree);
    let mat1: Matrix<ModInteger32> = generate_random_modint32_polynomial_matrix(num_rows, num_cols, ring_degree);
    let mat2: Matrix<ModInteger32> = generate_random_modint32_polynomial_matrix(num_rows, num_cols, ring_degree);

    // Test that v * (m1 + m2) = v * m1 + v * m2
    let sum1 = mat1.add(&mat2);
    let total1 = sum1.multiply_by_left_vector(&vector);

    let prod1 = mat1.multiply_by_left_vector(&vector);
    let prod2 = mat2.multiply_by_left_vector(&vector);
    let total2 = prod1.add(&prod2);

    assert_eq!(total1.ring_degree, total2.ring_degree);
    assert_eq!(total1.length, total2.length);
    for i in 0..total1.length {
        assert_eq!(total1.polys[i].coeffs, total2.polys[i].coeffs);
    }
}

#[test]
fn test_distributive_matrix_modint64() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let num_rows = rng.gen_range(1..=MAX_TEST_DEGREE);
    let num_cols = rng.gen_range(1..=MAX_TEST_DEGREE);
    let vector: Vector<ModInteger64> = generate_random_modint64_polynomial_vector(num_rows, ring_degree);
    let mat1: Matrix<ModInteger64> = generate_random_modint64_polynomial_matrix(num_rows, num_cols, ring_degree);
    let mat2: Matrix<ModInteger64> = generate_random_modint64_polynomial_matrix(num_rows, num_cols, ring_degree);

    // Test that v * (m1 + m2) = v * m1 + v * m2
    let sum1 = mat1.add(&mat2);
    let total1 = sum1.multiply_by_left_vector(&vector);

    let prod1 = mat1.multiply_by_left_vector(&vector);
    let prod2 = mat2.multiply_by_left_vector(&vector);
    let total2 = prod1.add(&prod2);

    assert_eq!(total1.ring_degree, total2.ring_degree);
    assert_eq!(total1.length, total2.length);
    for i in 0..total1.length {
        assert_eq!(total1.polys[i].coeffs, total2.polys[i].coeffs);
    }
}
