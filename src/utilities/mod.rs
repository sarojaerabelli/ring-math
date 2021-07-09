use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::ops::{Add, AddAssign, Sub, Mul};
use std::fmt::Debug;
use std::cmp::PartialEq;
use crate::polynomial::Polynomial;
use crate::vector::{Vector, Matrix};
use crate::ring::{Complex, ModInteger32, ModInteger64};
use crate::traits::{Zero, One, Abs};

pub fn check_vecs_almost_equal<T>(vec1: &Vec<T>, vec2: &Vec<T>, percent_error: f64) -> bool 
        where T: Copy + Sub<Output = T> + Debug + Abs<T>{
    if vec1.len() != vec2.len() {
        return false;
    }

    for i in 0..vec1.len() {
        if T::abs(vec1[i] - vec2[i]) / T::abs(vec1[i]) > percent_error {
            println!("vec1[{:?}] = {:?} and vec2[{:?}] = {:?}", i, vec1[i], i, vec2[i]);
            return false;
        }
    }

    true
}

pub fn generate_random_float_polynomial<T>(size: usize) -> Polynomial<T> 
        where Standard: Distribution<T>, T: Mul<Output = T> + Add + Sub + Zero<T> +
        One<T> + Sub<Output = T> + Copy + Add<Output = T> + Abs<T> + Debug + PartialEq + AddAssign {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<T> = Vec::new();
    for _ in 0..size {
        rand_vec.push(rng.gen::<T>());
    }
    Polynomial{ring_degree: size, coeffs: rand_vec}
}

pub fn generate_random_complex_polynomial<T>(size: usize) -> Polynomial<Complex<T>> 
        where Standard: Distribution<T>, T: Mul<Output = T> + Add + Sub + Zero<T> +
        One<T> + Sub<Output = T> + Copy + Add<Output = T> + Abs<T> + Debug + PartialEq + AddAssign {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<Complex<T>> = Vec::new();
    for _ in 0..size {
        let real = rng.gen::<T>();
        let imag = rng.gen::<T>();
        rand_vec.push(Complex{real, imag});
    }
    Polynomial{ring_degree: size, coeffs: rand_vec}
}

pub fn generate_random_modint32_polynomial(size: usize) -> Polynomial<ModInteger32> {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<ModInteger32> = Vec::new();
    for _ in 0..size {
        rand_vec.push(ModInteger32{value: rng.gen_range(0..(2^32))});
    }
    Polynomial{ring_degree: size, coeffs: rand_vec}
}

pub fn generate_random_modint64_polynomial(size: usize) -> Polynomial<ModInteger64> {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<ModInteger64> = Vec::new();
    for _ in 0..size {
        rand_vec.push(ModInteger64{value: rng.gen_range(0..(2^64))});
    }
    Polynomial{ring_degree: size, coeffs: rand_vec}
}

pub fn generate_random_float_polynomial_vector<T>(size: usize, ring_degree: usize) -> Vector<T> 
        where Standard: Distribution<T>, T: Mul<Output = T> + Add + Sub + Zero<T> +
        One<T> + Sub<Output = T> + Copy + Add<Output = T> + Abs<T> + Debug + PartialEq + AddAssign {
    let mut rand_poly_vec: Vec<Polynomial<T>> = Vec::new();
    for _ in 0..size {
        let rand_poly: Polynomial<T> = generate_random_float_polynomial(ring_degree);
        rand_poly_vec.push(rand_poly);
    }
    Vector{ring_degree, length: size, polys: rand_poly_vec}
}

pub fn generate_random_complex_polynomial_vector<T>(
    size: usize, ring_degree: usize) -> Vector<Complex<T>> 
        where Standard: Distribution<T>, T: Mul<Output = T> + Add + Sub + Zero<T> +
        One<T> + Sub<Output = T> + Copy + Add<Output = T> + Abs<T> + Debug + PartialEq + AddAssign {
    let mut rand_poly_vec: Vec<Polynomial<Complex<T>>> = Vec::new();
    for _ in 0..size {
        let rand_poly: Polynomial<Complex<T>> = generate_random_complex_polynomial(ring_degree);
        rand_poly_vec.push(rand_poly);
    }
    Vector{ring_degree, length: size, polys: rand_poly_vec}
}

pub fn generate_random_modint32_polynomial_vector(
    size: usize, ring_degree: usize) -> Vector<ModInteger32> {
    let mut rand_poly_vec: Vec<Polynomial<ModInteger32>> = Vec::new();
    for _ in 0..size {
        let rand_poly: Polynomial<ModInteger32> = generate_random_modint32_polynomial(ring_degree);
        rand_poly_vec.push(rand_poly);
    }
    Vector{ring_degree, length: size, polys: rand_poly_vec}
}

pub fn generate_random_modint64_polynomial_vector(
    size: usize, ring_degree: usize) -> Vector<ModInteger64> {
    let mut rand_poly_vec: Vec<Polynomial<ModInteger64>> = Vec::new();
    for _ in 0..size {
        let rand_poly: Polynomial<ModInteger64> = generate_random_modint64_polynomial(ring_degree);
        rand_poly_vec.push(rand_poly);
    }
    Vector{ring_degree, length: size, polys: rand_poly_vec}
}

pub fn generate_random_float_polynomial_matrix<T>(
    num_rows: usize, num_cols: usize, ring_degree: usize) -> Matrix<T> 
        where Standard: Distribution<T>, T: Mul<Output = T> + Add + Sub + Zero<T> +
        One<T> + Sub<Output = T> + Copy + Add<Output = T> + Abs<T> + Debug + PartialEq + AddAssign {
    let mut rand_poly_matrix: Vec<Vector<T>> = Vec::new();
    for _ in 0..num_cols {
        let rand_poly_vec: Vector<T> = generate_random_float_polynomial_vector(num_rows, ring_degree);
        rand_poly_matrix.push(rand_poly_vec);
    }
    Matrix{ring_degree, num_rows, num_cols, cols: rand_poly_matrix}
}

pub fn generate_random_complex_polynomial_matrix<T>(
    num_rows: usize, num_cols: usize, ring_degree: usize) -> Matrix<Complex<T>> 
        where Standard: Distribution<T>, T: Mul<Output = T> + Add + Sub + Zero<T> +
        One<T> + Sub<Output = T> + Copy + Add<Output = T> + Abs<T> + Debug + PartialEq + AddAssign {
    let mut rand_poly_matrix: Vec<Vector<Complex<T>>> = Vec::new();
    for _ in 0..num_cols {
        let rand_poly_vec: Vector<Complex<T>> = generate_random_complex_polynomial_vector(num_rows, ring_degree);
        rand_poly_matrix.push(rand_poly_vec);
    }
    Matrix{ring_degree, num_rows, num_cols, cols: rand_poly_matrix}
}

pub fn generate_random_modint32_polynomial_matrix(
    num_rows: usize, num_cols: usize, ring_degree: usize) -> Matrix<ModInteger32> {
    let mut rand_poly_matrix: Vec<Vector<ModInteger32>> = Vec::new();
    for _ in 0..num_cols {
        let rand_poly_vec: Vector<ModInteger32> = generate_random_modint32_polynomial_vector(num_rows, ring_degree);
        rand_poly_matrix.push(rand_poly_vec);
    }
    Matrix{ring_degree, num_rows, num_cols, cols: rand_poly_matrix}
}

pub fn generate_random_modint64_polynomial_matrix(
    num_rows: usize, num_cols: usize, ring_degree: usize) -> Matrix<ModInteger64> {
    let mut rand_poly_matrix: Vec<Vector<ModInteger64>> = Vec::new();
    for _ in 0..num_cols {
        let rand_poly_vec: Vector<ModInteger64> = generate_random_modint64_polynomial_vector(num_rows, ring_degree);
        rand_poly_matrix.push(rand_poly_vec);
    }
    Matrix{ring_degree, num_rows, num_cols, cols: rand_poly_matrix}
}
