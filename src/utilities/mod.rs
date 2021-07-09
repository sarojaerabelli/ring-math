use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::ops::{Add, Sub, Mul};
use std::fmt::Debug;
use crate::ring::{Complex, ModInteger32, ModInteger64};
use crate::traits::{Zero, Abs};

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
        Sub<Output = T> + Copy + Add<Output = T> + Abs<T> + Debug {
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