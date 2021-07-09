// Tests
use super::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use crate::utilities::{check_vecs_almost_equal, generate_random_float_polynomial,
                       generate_random_complex_polynomial, generate_random_modint32_polynomial,
                       generate_random_modint64_polynomial};
use crate::ring::{Complex, ModInteger32, ModInteger64};
use crate::traits::{Abs, Zero, One};
use std::ops::{Sub, Div};
use std::cmp::{PartialOrd, PartialEq};
use std::fmt::Debug;

const MAX_TEST_DEGREE: usize = 2048;
const F32_ADD_ERROR: f64 = 0.000001;
const F32_MULTIPLY_ERROR: f64 = 0.01;
const F64_ERROR: f64 = 0.0000000001;
/*
#[test]
#[should_panic(expected = "Ring degree should be equal to vector length. 10 != 9")]
fn test_check_coeff_length() {
    let poly = Polynomial {
        ring_degree: 10,
        coeffs: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
    };
    poly.check_coeff_length();
}

#[test]
fn test_add_known_answer() {
    let poly1 = Polynomial {
        ring_degree: 10,
        coeffs: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    };
    let poly2 = Polynomial {
        ring_degree: 10,
        coeffs: vec![4.0, 5.0, 4.0, 8.0, 9.0, 1.0, 1.0, 1.0, 1.0, 3.0],
    };

    let sum = poly1.add(&poly2);

    assert_eq!(sum.ring_degree, poly1.ring_degree);
    assert_eq!(sum.coeffs, vec![5.0, 7.0, 7.0, 12.0, 14.0, 7.0, 8.0, 9.0, 10.0, 13.0]);
}

fn test_distributive_float<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> + One<T> + PartialEq {

    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<T> = generate_random_float_polynomial::<T>(ring_degree);
    let rand_vec2: Vec<T> = generate_random_float_polynomial::<T>(ring_degree);
    let rand_vec3: Vec<T> = generate_random_float_polynomial::<T>(ring_degree);

    let poly1 = Polynomial {
        ring_degree,
        coeffs: rand_vec1,
    };
    let poly2 = Polynomial {
        ring_degree,
        coeffs: rand_vec2,
    };
    let poly3 = Polynomial {
        ring_degree,
        coeffs: rand_vec3,
    };

    // Test that p1 * (p2 + p3) = p1 * p2 + p1 * p3
    let sum1 = poly2.add(&poly3);
    let total1 = poly1.multiply(&sum1);

    let prod1 = poly1.multiply(&poly2);
    let prod2 = poly1.multiply(&poly3);
    let total2 = prod1.add(&prod2);

    assert_eq!(total1.ring_degree, total2.ring_degree);
    assert!(check_vecs_almost_equal(total1.coeffs, total2.coeffs, error));
}

#[test]
fn test_distributive_float_f32() {
    test_distributive_float::<f32>(F32_MULTIPLY_ERROR);
}

#[test]
fn test_distributive_float_f64() {
    test_distributive_float::<f64>(F64_ERROR);
}

fn test_distributive_complex<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> + One<T> + PartialEq {

    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<Complex<T>> = generate_random_complex_polynomial::<T>(ring_degree);
    let rand_vec2: Vec<Complex<T>> = generate_random_complex_polynomial::<T>(ring_degree);
    let rand_vec3: Vec<Complex<T>> = generate_random_complex_polynomial::<T>(ring_degree);

    let poly1 = Polynomial {
        ring_degree,
        coeffs: rand_vec1,
    };
    let poly2 = Polynomial {
        ring_degree,
        coeffs: rand_vec2,
    };
    let poly3 = Polynomial {
        ring_degree,
        coeffs: rand_vec3,
    };

    // Test that p1 * (p2 + p3) = p1 * p2 + p1 * p3
    let sum1 = poly2.add(&poly3);
    let total1 = poly1.multiply(&sum1);

    let prod1 = poly1.multiply(&poly2);
    let prod2 = poly1.multiply(&poly3);
    let total2 = prod1.add(&prod2);

    assert_eq!(total1.ring_degree, total2.ring_degree);
    assert!(check_vecs_almost_equal(total1.coeffs, total2.coeffs, error));
}

#[test]
fn test_distributive_complex_f32() {
    test_distributive_complex::<f32>(F32_MULTIPLY_ERROR);
}

#[test]
fn test_distributive_complex_f64() {
    test_distributive_complex::<f64>(F64_ERROR);
}

#[test]
fn test_distributive_modint32() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<ModInteger32> = generate_random_modint32_polynomial(ring_degree);
    let rand_vec2: Vec<ModInteger32> = generate_random_modint32_polynomial(ring_degree);
    let rand_vec3: Vec<ModInteger32> = generate_random_modint32_polynomial(ring_degree);

    let poly1 = Polynomial {
        ring_degree,
        coeffs: rand_vec1,
    };
    let poly2 = Polynomial {
        ring_degree,
        coeffs: rand_vec2,
    };

    let poly3 = Polynomial {
        ring_degree,
        coeffs: rand_vec3,
    };

   // Test that p1 * (p2 + p3) = p1 * p2 + p1 * p3
    let sum1 = poly2.add(&poly3);
    let total1 = poly1.multiply(&sum1);

    let prod1 = poly1.multiply(&poly2);
    let prod2 = poly1.multiply(&poly3);
    let total2 = prod1.add(&prod2);

    assert_eq!(total1.ring_degree, total2.ring_degree);
    assert_eq!(total1.coeffs, total2.coeffs);
}

#[test]
fn test_distributive_modint64() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<ModInteger64> = generate_random_modint64_polynomial(ring_degree);
    let rand_vec2: Vec<ModInteger64> = generate_random_modint64_polynomial(ring_degree);
    let rand_vec3: Vec<ModInteger64> = generate_random_modint64_polynomial(ring_degree);

    let poly1 = Polynomial {
        ring_degree,
        coeffs: rand_vec1,
    };
    let poly2 = Polynomial {
        ring_degree,
        coeffs: rand_vec2,
    };

    let poly3 = Polynomial {
        ring_degree,
        coeffs: rand_vec3,
    };

   // Test that p1 * (p2 + p3) = p1 * p2 + p1 * p3
    let sum1 = poly2.add(&poly3);
    let total1 = poly1.multiply(&sum1);

    let prod1 = poly1.multiply(&poly2);
    let prod2 = poly1.multiply(&poly3);
    let total2 = prod1.add(&prod2);

    assert_eq!(total1.ring_degree, total2.ring_degree);
    assert_eq!(total1.coeffs, total2.coeffs);
}
*/
