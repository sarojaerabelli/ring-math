use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::cmp::PartialOrd;
use std::ops::{Sub, Mul, Div};
use std::fmt::Debug;

pub trait Abs<T> {
    fn abs(self) -> f64;
}

impl Abs<f32> for f32 {
    fn abs(self) -> f64 {
        if self > 0.0 {
            return self as f64;
        } else {
            return -self as f64;
        }
    }
}

impl Abs<f64> for f64 {
    fn abs(self) -> f64 {
        if self > 0.0 {
            return self;
        } else {
            return -self;
        }
    }
}

pub fn generate_random_float_vector<T>(size: usize) -> Vec<T> 
        where Standard: Distribution<T>, T: Mul<Output = T> {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<T> = Vec::new();
    for _ in 0..size {
        rand_vec.push(rng.gen::<T>());
    }
    rand_vec
}

pub fn check_lists_almost_equal<T>(vec1: Vec<T>, vec2: Vec<T>, percent_error: f64) -> bool 
        where T: PartialOrd + Copy + Sub<Output = T> + Div<Output = T> + Debug + Div + Abs<T> + Div<Output = T> {
    if vec1.len() != vec2.len() {
        return false;
    }

    for i in 0..vec1.len() {
        if T::abs((vec1[i] - vec2[i]) / vec1[i]) > percent_error {
            println!("vec1[{:?}] = {:?} and vec2[{:?}] = {:?}", i, vec1[i], i, vec2[i]);
            return false;
        }
    }

    true
}