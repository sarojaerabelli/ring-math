// Tests
use super::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

fn generate_random_float_vector<T>(size: usize) -> Vec<T> 
        where Standard: Distribution<T>, T: std::ops::Mul<Output = T> {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<T> = Vec::new();
    for i in 0..size {
        rand_vec.push(rng.gen::<T>());
    }
    rand_vec
}

#[test]
fn test_add() {
    let coeffs: Vec<f64> = generate_random_float_vector(10);
    println!("Random coeffs: {:?}", coeffs);
    let num: u32 = u32::pow(2, 31) * 2 - 1;
    println!("Num: {}", num);
    let poly1 = Polynomial {
        ring_degree: 10,
        coeffs: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };
    let poly2 = Polynomial {
        ring_degree: 10,
        coeffs: vec![4, 5, 4, 8, 9, 1, 1, 1, 1, 3],
    };

    let sum = poly1.add(&poly2);

    assert_eq!(sum.ring_degree, poly1.ring_degree);
    assert_eq!(sum.coeffs, vec![5, 7, 7, 12, 14, 7, 8, 9, 10, 13]);
}

#[test]
#[should_panic(expected = "Ring degrees should be equal. 10 != 14")]
fn test_add_different_ring_degrees() {
    let poly1 = Polynomial {
        ring_degree: 10,
        coeffs: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };
    let poly2 = Polynomial {
        ring_degree: 14,
        coeffs: vec![4, 5, 4, 8, 9, 1, 1, 1, 1, 3],
    };

    // Should panic because the polynomials have different ring degrees.
    poly1.add(&poly2);
}