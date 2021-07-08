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

#[test]
fn test_add() {
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
fn test_add_different_ring_degrees() {
    let poly1 = Polynomial {
        ring_degree: 10,
        coeffs: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    };
    let poly2 = Polynomial {
        ring_degree: 14,
        coeffs: vec![4.0, 5.0, 4.0, 8.0, 9.0, 1.0, 1.0, 1.0, 1.0, 3.0],
    };

    // Should panic because the polynomials have different ring degrees.
    poly1.add(&poly2);
}

#[test]
fn test_multiply() {
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