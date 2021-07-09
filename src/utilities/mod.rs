use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::ops::{Add, Sub, Mul};
use std::fmt::Debug;
use std::cmp::PartialEq;
use crate::ring::{Complex, ModInteger32, ModInteger64};
use crate::traits::{Zero, One, Abs};

pub fn check_vecs_almost_equal<T>(vec1: Vec<T>, vec2: Vec<T>, percent_error: f64) -> bool 
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

pub fn generate_random_float_vector<T>(size: usize) -> Vec<T> 
        where Standard: Distribution<T>, T: Mul<Output = T> {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<T> = Vec::new();
    for _ in 0..size {
        rand_vec.push(rng.gen::<T>());
    }
    rand_vec
}

pub fn generate_random_complex_vector<T>(size: usize) -> Vec<Complex<T>> 
        where Standard: Distribution<T>, T: Mul<Output = T> + Add + Sub + Zero<T> +
        One<T> + Sub<Output = T> + Copy + Add<Output = T> + Abs<T> + Debug + PartialEq {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<Complex<T>> = Vec::new();
    for _ in 0..size {
        let real = rng.gen::<T>();
        let imag = rng.gen::<T>();
        rand_vec.push(Complex{real, imag});
    }
    rand_vec
}

pub fn generate_random_modint32_vector(size: usize) -> Vec<ModInteger32> {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<ModInteger32> = Vec::new();
    for _ in 0..size {
        rand_vec.push(ModInteger32{value: rng.gen_range(0..(2^32))});
    }
    rand_vec
}

pub fn generate_random_modint64_vector(size: usize) -> Vec<ModInteger64> {
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<ModInteger64> = Vec::new();
    for _ in 0..size {
        rand_vec.push(ModInteger64{value: rng.gen_range(0..(2^64))});
    }
    rand_vec
}