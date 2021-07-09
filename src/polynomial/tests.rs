// Tests
use super::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use crate::utilities::{check_vecs_almost_equal, generate_random_float_vector,
                       generate_random_complex_vector, generate_random_modint32_vector,
                       generate_random_modint64_vector};
use crate::ring::{Complex, ModInteger32, ModInteger64};
use crate::traits::{Abs, Zero};
use std::ops::{Sub, Div};
use std::cmp::PartialOrd;
use std::fmt::Debug;

const MAX_TEST_DEGREE: usize = 2048;
const F32_ADD_ERROR: f64 = 0.000001;
const F32_MULTIPLY_ERROR: f64 = 0.01;
const F64_ERROR: f64 = 0.0000000001;

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

fn test_add_different_ring_degrees_fail<T>() 
        where T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign{
    let poly1: Polynomial<T> = Polynomial::new(10);
    let poly2: Polynomial<T> = Polynomial::new(14);

    // Should panic because the polynomials have different ring degrees.
    poly1.add(&poly2);
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_add_different_ring_degrees_fail_f32() {
    test_add_different_ring_degrees_fail::<f32>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_add_different_ring_degrees_fail_f64() {
    test_add_different_ring_degrees_fail::<f64>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_add_different_ring_degrees_fail_complex_f32() {
    test_add_different_ring_degrees_fail::<Complex<f32>>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_add_different_ring_degrees_fail_complex_f64() {
    test_add_different_ring_degrees_fail::<Complex<f64>>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_add_different_ring_degrees_fail_modint32() {
    test_add_different_ring_degrees_fail::<ModInteger32>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_add_different_ring_degrees_fail_modint64() {
    test_add_different_ring_degrees_fail::<ModInteger64>();
}

fn test_add_commutative_float<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd  + Debug +
        Div + Abs<T> + Div<Output = T>{
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let rand_vec2: Vec<T> = generate_random_float_vector::<T>(ring_degree);

    let poly1 = Polynomial {
        ring_degree,
        coeffs: rand_vec1,
    };
    let poly2 = Polynomial {
        ring_degree,
        coeffs: rand_vec2,
    };

    // Test that p1 + p2 = p2 + p1.
    let sum1 = poly1.add(&poly2);
    let sum2 = poly2.add(&poly1);

    assert_eq!(sum1.ring_degree, sum2.ring_degree);
    assert!(check_vecs_almost_equal(sum1.coeffs, sum2.coeffs, error));
}

#[test]
fn test_add_commutative_float_f32() {
    test_add_commutative_float::<f32>(F32_ADD_ERROR);
}

#[test]
fn test_add_commutative_float_f64() {
    test_add_commutative_float::<f64>(F64_ERROR);
}

fn test_add_commutative_complex<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd  + std::fmt::Debug +
        Div + Abs<T> + Div<Output = T> {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let rand_vec2: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);

    let poly1 = Polynomial {
        ring_degree,
        coeffs: rand_vec1,
    };
    let poly2 = Polynomial {
        ring_degree,
        coeffs: rand_vec2,
    };

    // Test that p1 + p2 = p2 + p1.
    let sum1 = poly1.add(&poly2);
    let sum2 = poly2.add(&poly1);

    assert_eq!(sum1.ring_degree, sum2.ring_degree);
    assert!(check_vecs_almost_equal(sum1.coeffs, sum2.coeffs, error));
}

#[test]
fn test_add_commutative_complex_f32() {
    test_add_commutative_complex::<f32>(F32_ADD_ERROR);
}

#[test]
fn test_add_commutative_complex_f64() {
    test_add_commutative_complex::<f64>(F64_ERROR);
}

fn test_add_associative_float<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T>{
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let rand_vec2: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let rand_vec3: Vec<T> = generate_random_float_vector::<T>(ring_degree);

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

    // Test that (p1 + p2) + p3 = p1 + (p2 + p3).
    let sum1 = poly1.add(&poly2);
    let total_sum1 = sum1.add(&poly3);

    let sum2 = poly2.add(&poly3);
    let total_sum2 = poly1.add(&sum2);

    assert_eq!(total_sum1.ring_degree, total_sum2.ring_degree);
    assert!(check_vecs_almost_equal(total_sum1.coeffs, total_sum2.coeffs, error));
}

#[test]
fn test_add_associative_float_f32() {
    test_add_associative_float::<f32>(F32_ADD_ERROR);
}

#[test]
fn test_add_associative_float_f64() {
    test_add_associative_float::<f64>(F64_ERROR);
}

fn test_add_associative_complex<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T>{
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let rand_vec2: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let rand_vec3: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);

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

    // Test that (p1 + p2) + p3 = p1 + (p2 + p3).
    let sum1 = poly1.add(&poly2);
    let total_sum1 = sum1.add(&poly3);

    let sum2 = poly2.add(&poly3);
    let total_sum2 = poly1.add(&sum2);

    assert_eq!(total_sum1.ring_degree, total_sum2.ring_degree);
    assert!(check_vecs_almost_equal(total_sum1.coeffs, total_sum2.coeffs, error));
}

#[test]
fn test_add_associative_complex_f32() {
    test_add_associative_complex::<f32>(F32_ADD_ERROR);
}

#[test]
fn test_add_associative_complex_f64() {
    test_add_associative_complex::<f64>(F64_ERROR);
}

#[test]
fn test_multiply_known_answer() {
    let poly1 = Polynomial {
        ring_degree: 10,
        coeffs: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    };
    let poly2 = Polynomial {
        ring_degree: 10,
        coeffs: vec![4.0, 5.0, 4.0, 8.0, 9.0, 1.0, 1.0, 1.0, 1.0, 3.0],
    };

    let sum = poly1.multiply(&poly2);

    assert_eq!(sum.ring_degree, poly1.ring_degree);
    assert_eq!(sum.coeffs, vec![241.0, 228.0, 225.0, 182.0, 129.0, 156.0, 183.0, 210.0,
                                237.0, 244.0]);
}

fn test_multiply_different_ring_degrees_fail<T>() 
        where T: Add<Output = T> + Mul<Output = T> + Copy + Zero<T> + AddAssign{
    let poly1: Polynomial<T> = Polynomial::new(10);
    let poly2: Polynomial<T> = Polynomial::new(14);

    // Should panic because the polynomials have different ring degrees.
    poly1.multiply(&poly2);
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_multiply_different_ring_degrees_fail_f32() {
    test_multiply_different_ring_degrees_fail::<f32>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_multiply_different_ring_degrees_fail_f64() {
    test_multiply_different_ring_degrees_fail::<f64>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_multiply_different_ring_degrees_fail_complex_f32() {
    test_multiply_different_ring_degrees_fail::<Complex<f32>>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_multiply_different_ring_degrees_fail_complex_f64() {
    test_multiply_different_ring_degrees_fail::<Complex<f64>>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_multiply_different_ring_degrees_fail_modint32() {
    test_multiply_different_ring_degrees_fail::<ModInteger32>();
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_multiply_different_ring_degrees_fail_modint64() {
    test_multiply_different_ring_degrees_fail::<ModInteger64>();
}

fn test_multiply_commutative_float<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let rand_vec2: Vec<T> = generate_random_float_vector::<T>(ring_degree);

    let poly1 = Polynomial {
        ring_degree,
        coeffs: rand_vec1,
    };
    let poly2 = Polynomial {
        ring_degree,
        coeffs: rand_vec2,
    };

    // Test that p1 * p2 = p2 * p1.
    let prod1 = poly1.multiply(&poly2);
    let prod2 = poly2.multiply(&poly1);

    assert_eq!(prod1.ring_degree, prod2.ring_degree);
    assert!(check_vecs_almost_equal(prod1.coeffs, prod2.coeffs, error));
}

#[test]
fn test_multiply_commutative_float_f32() {
    test_multiply_commutative_float::<f32>(F32_MULTIPLY_ERROR);
}

#[test]
fn test_multiply_commutative_float_f64() {
    test_multiply_commutative_float::<f64>(F64_ERROR);
}

fn test_multiply_commutative_complex<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let rand_vec2: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);

    let poly1 = Polynomial {
        ring_degree,
        coeffs: rand_vec1,
    };
    let poly2 = Polynomial {
        ring_degree,
        coeffs: rand_vec2,
    };

    // Test that p1 * p2 = p2 * p1.
    let prod1 = poly1.multiply(&poly2);
    let prod2 = poly2.multiply(&poly1);

    assert_eq!(prod1.ring_degree, prod2.ring_degree);
    assert!(check_vecs_almost_equal(prod1.coeffs, prod2.coeffs, error));
}

#[test]
fn test_multiply_commutative_complex_f32() {
    test_multiply_commutative_complex::<f32>(F32_MULTIPLY_ERROR);
}

#[test]
fn test_multiply_commutative_complex_f64() {
    test_multiply_commutative_complex::<f64>(F64_ERROR);
}

fn test_multiply_associative_float<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let rand_vec2: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let rand_vec3: Vec<T> = generate_random_float_vector::<T>(ring_degree);

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

    // Test that (p1 * p2) * p3 = p1 * (p2 * p3).
    let prod1 = poly1.multiply(&poly2);
    let total_prod1 = prod1.multiply(&poly3);

    let prod2 = poly2.multiply(&poly3);
    let total_prod2 = poly1.multiply(&prod2);

    assert_eq!(total_prod1.ring_degree, total_prod2.ring_degree);
    assert!(check_vecs_almost_equal(total_prod1.coeffs, total_prod2.coeffs, error));
}

#[test]
fn test_multiply_associative_float_f32() {
    test_multiply_associative_float::<f32>(F32_MULTIPLY_ERROR);
}

#[test]
fn test_multiply_associative_float_f64() {
    test_multiply_associative_float::<f64>(F64_ERROR);
}

fn test_multiply_associative_complex<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let rand_vec2: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let rand_vec3: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);

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

    // Test that (p1 * p2) * p3 = p1 * (p2 * p3).
    let prod1 = poly1.multiply(&poly2);
    let total_prod1 = prod1.multiply(&poly3);

    let prod2 = poly2.multiply(&poly3);
    let total_prod2 = poly1.multiply(&prod2);

    assert_eq!(total_prod1.ring_degree, total_prod2.ring_degree);
    assert!(check_vecs_almost_equal(total_prod1.coeffs, total_prod2.coeffs, error));
}

#[test]
fn test_multiply_associative_complex_f32() {
    test_multiply_associative_complex::<f32>(F32_MULTIPLY_ERROR);
}

#[test]
fn test_multiply_associative_complex_f64() {
    test_multiply_associative_complex::<f64>(F64_ERROR);
}

fn test_distributive_float<T>(error: f64)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd + Debug +
        Div + Abs<T> + Div<Output = T> {

    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let rand_vec2: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let rand_vec3: Vec<T> = generate_random_float_vector::<T>(ring_degree);

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
        Div + Abs<T> + Div<Output = T> {

    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=MAX_TEST_DEGREE);
    let rand_vec1: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let rand_vec2: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let rand_vec3: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);

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
fn test_multiply_by_x_known_answer() {
    let poly = Polynomial {
        ring_degree: 10,
        coeffs: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    };

    let prod = poly.multiply_by_x();

    assert_eq!(prod.ring_degree, poly.ring_degree);
    assert_eq!(prod.coeffs, vec![10.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
}

#[test]
fn test_multiply_by_x_compare_multiply() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=2048);
    let rand_vec: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let poly = Polynomial {
        ring_degree,
        coeffs: rand_vec,
    };

    let mut x_vec: Vec<f64> = vec![0.0; ring_degree];
    x_vec[1] = 1.0;
    let x_poly = Polynomial {
        ring_degree,
        coeffs: x_vec,
    };

    let prod1 = poly.multiply_by_x();
    let prod2 = poly.multiply(&x_poly);

    assert_eq!(prod1.ring_degree, prod2.ring_degree);
    assert_eq!(prod1.coeffs, prod2.coeffs);
}

