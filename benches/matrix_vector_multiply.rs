use criterion::{black_box, Criterion};
use rand::{distributions::Standard, prelude::Distribution};
use zama_math::vector::{Vector, Matrix};
use zama_math::ring::{Complex, ModInteger32, ModInteger64};
use zama_math::traits::{Abs, Zero, One};
use std::ops::{Add, AddAssign, Sub, Mul, Div};
use std::cmp::{PartialOrd, PartialEq};
use std::fmt::Debug;
use zama_math::utilities::{generate_random_float_polynomial_vector, generate_random_complex_polynomial_vector,
                           generate_random_modint32_polynomial_vector, generate_random_modint64_polynomial_vector,
                           generate_random_float_polynomial_matrix, generate_random_complex_polynomial_matrix,
                           generate_random_modint32_polynomial_matrix, generate_random_modint64_polynomial_matrix};

pub fn bench_float<T>(c: &mut Criterion, ring_degree: usize, num_rows: usize, num_cols: usize)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd  + Debug +
        Div + Abs<T> + Div<Output = T> + One<T> + PartialEq {
    let name = format!("Multiply vector by matrix of degree {} with {} rows and {} cols of type {}", ring_degree,
                       num_rows, num_cols, std::any::type_name::<T>());
    let vector: Vector<T> = generate_random_float_polynomial_vector::<T>(num_rows, ring_degree);
    let matrix: Matrix<T> = generate_random_float_polynomial_matrix::<T>(num_rows, num_cols, ring_degree);
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(matrix.multiply_by_left_vector(&vector));
        })
    });
}

pub fn bench_complex<T>(c: &mut Criterion, ring_degree: usize, num_rows: usize, num_cols: usize)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd  + Debug +
        Div + Abs<T> + Div<Output = T> + One<T> + PartialEq {
    let name = format!("Multiply vector by matrix of degree {} with {} rows and {} cols of type {}", ring_degree,
                       num_rows, num_cols, std::any::type_name::<Complex<T>>());
    let vector: Vector<Complex<T>> = generate_random_complex_polynomial_vector::<T>(num_rows, ring_degree);
    let matrix: Matrix<Complex<T>> = generate_random_complex_polynomial_matrix::<T>(num_rows, num_cols, ring_degree);
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(matrix.multiply_by_left_vector(&vector));
        })
    });
}

pub fn bench_modint32(c: &mut Criterion, ring_degree: usize, num_rows: usize, num_cols: usize) {
    let name = format!("Multiply vector by matrix of degree {} with {} rows and {} cols of type {}", ring_degree,
                       num_rows, num_cols, std::any::type_name::<ModInteger32>());
    let vector: Vector<ModInteger32> = generate_random_modint32_polynomial_vector(num_rows, ring_degree);
    let matrix: Matrix<ModInteger32> = generate_random_modint32_polynomial_matrix(num_rows, num_cols, ring_degree);
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(matrix.multiply_by_left_vector(&vector));
        })
    });
}

pub fn bench_modint64(c: &mut Criterion, ring_degree: usize, num_rows: usize, num_cols: usize) {
    let name = format!("Multiply vector by matrix of degree {} with {} rows and {} cols of type {}", ring_degree,
                       num_rows, num_cols, std::any::type_name::<ModInteger64>());
    let vector: Vector<ModInteger64> = generate_random_modint64_polynomial_vector(num_rows, ring_degree);
    let matrix: Matrix<ModInteger64> = generate_random_modint64_polynomial_matrix(num_rows, num_cols, ring_degree);
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(matrix.multiply_by_left_vector(&vector));
        })
    });
}

pub fn bench_f32_degree_100_rows_10_cols_10(c: &mut Criterion) {
    bench_float::<f32>(c, 100, 10, 10);
}

pub fn bench_f64_degree_100_rows_10_cols_10(c: &mut Criterion) {
    bench_float::<f64>(c, 100, 10, 10);
}

pub fn bench_complex_f32_degree_100_rows_10_cols_10(c: &mut Criterion) {
    bench_complex::<f32>(c, 100, 10, 10);
}

pub fn bench_complex_f64_degree_100_rows_10_cols_10(c: &mut Criterion) {
    bench_complex::<f64>(c, 100, 10, 10);
}

pub fn bench_modint32_degree_100_rows_10_cols_10(c: &mut Criterion) {
    bench_modint32(c, 100, 10, 10);
}

pub fn bench_modint64_degree_100_rows_10_cols_10(c: &mut Criterion) {
    bench_modint64(c, 100, 10, 10);
}
