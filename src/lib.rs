// Polynomial library
use std::ops::Add;

struct Polynomial<T: Add<Output = T> + Copy> {
    ring_degree: usize,
    coeffs: Vec<T>
}

impl<T: Add<Output = T> + Copy> Polynomial<T> {

    fn add(&self, other: &Polynomial<T>) -> Polynomial<T> {
        let mut sum: Vec<T> = Vec::new();
        for i in 0..self.ring_degree {
            sum.push(self.coeffs[i] + other.coeffs[i]);
        }
        Polynomial {ring_degree: self.ring_degree,
                    coeffs: sum}
    }

}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
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
}