// Tests
use super::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

fn generate_random_float_vector<T>(size: usize) -> Vec<T> 
        where Standard: Distribution<T>, T: std::ops::Mul<Output = T> {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<T> = Vec::new();
    for _ in 0..size {
        rand_vec.push(rng.gen::<T>());
    }
    rand_vec
}

fn check_lists_almost_equal<T>(vec1: Vec<T>, vec2: Vec<T>, error: T) -> bool 
        where T: std::cmp::PartialOrd + Copy + std::ops::Sub<Output = T> {
    if vec1.len() != vec2.len() {
        return false;
    }

    for i in 0..vec1.len() {
        if vec1[i] - vec2[i] > error {
            return false;
        } else if vec2[i] - vec1[i] > error {
            return false;
        }
    }

    true
}

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

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_add_different_ring_degrees_fail() {
    let poly1: Polynomial<f64> = Polynomial::new(10);
    let poly2: Polynomial<f64> = Polynomial::new(14);

    // Should panic because the polynomials have different ring degrees.
    poly1.add(&poly2);
}

#[test]
fn test_add_commutative() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=2048);
    let rand_vec1: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let rand_vec2: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);

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
    assert!(check_lists_almost_equal(sum1.coeffs, sum2.coeffs, 0.00000001));
}

#[test]
fn test_add_associative() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=2048);
    let rand_vec1: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let rand_vec2: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let rand_vec3: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);

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
    assert!(check_lists_almost_equal(total_sum1.coeffs, total_sum2.coeffs, 0.00000001));
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
    assert_eq!(sum.coeffs, vec![241.0, 228.0, 225.0, 182.0, 129.0, 156.0, 183.0, 210.0, 237.0, 244.0]);
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_multiply_different_ring_degrees_fail() {
    let poly1: Polynomial<f64> = Polynomial::new(10);
    let poly2: Polynomial<f64> = Polynomial::new(14);

    // Should panic because the polynomials have different ring degrees.
    poly1.multiply(&poly2);
}

#[test]
fn test_multiply_commutative() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=2048);
    let rand_vec1: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let rand_vec2: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);

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
    assert!(check_lists_almost_equal(prod1.coeffs, prod2.coeffs, 0.00000001));
}

#[test]
fn test_multiply_associative() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=2048);
    let rand_vec1: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let rand_vec2: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let rand_vec3: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);

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
    assert!(check_lists_almost_equal(total_prod1.coeffs, total_prod2.coeffs, 0.00000001));
}

#[test]
fn test_distributive() {
    // Generate random polynomials.
    let mut rng = rand::thread_rng();
    let ring_degree = rng.gen_range(1..=2048);
    let rand_vec1: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let rand_vec2: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
    let rand_vec3: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);

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
    assert!(check_lists_almost_equal(total1.coeffs, total2.coeffs, 0.00000001));
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
