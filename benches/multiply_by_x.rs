use criterion::{black_box, Criterion};
use rand::{distributions::Standard, prelude::Distribution};
use zama_math::polynomial::Polynomial;
use zama_math::ring::{Complex, ModInteger32, ModInteger64};
use zama_math::traits::{Abs, Zero, One};
use std::ops::{Add, AddAssign, Sub, Mul, Div};
use std::cmp::{PartialOrd, PartialEq};
use std::fmt::Debug;
use zama_math::utilities::{generate_random_float_vector, generate_random_complex_vector,
                           generate_random_modint32_vector, generate_random_modint64_vector};

pub fn bench_float<T>(c: &mut Criterion, ring_degree: usize)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd  + Debug +
        Div + Abs<T> + Div<Output = T> + One<T> + PartialEq {
    let name = format!("Add polynomials of degree {} with type {}", ring_degree,
                       std::any::type_name::<T>());
    let rand_vec: Vec<T> = generate_random_float_vector::<T>(ring_degree);
    let poly = Polynomial {
        ring_degree,
        coeffs: rand_vec,
    };
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(poly.multiply_by_x());
        })
    });
}

pub fn bench_complex<T>(c: &mut Criterion, ring_degree: usize)
        where Standard: Distribution<T>, T: Add<Output = T> + Mul<Output = T> + Copy +
        Zero<T> + AddAssign + Sub + Sub<Output = T> + PartialOrd  + Debug +
        Div + Abs<T> + Div<Output = T> + One<T> + PartialEq {
    let name = format!("Add polynomials of degree {} with type {}", ring_degree,
                       std::any::type_name::<Complex<T>>());
    let rand_vec: Vec<Complex<T>> = generate_random_complex_vector::<T>(ring_degree);
    let poly = Polynomial {
        ring_degree,
        coeffs: rand_vec,
    };
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(poly.multiply_by_x());
        })
    });
}

pub fn bench_modint32(c: &mut Criterion, ring_degree: usize) {
    let name = format!("Add polynomials of degree {} with type {}", ring_degree,
                       std::any::type_name::<ModInteger32>());
    let rand_vec: Vec<ModInteger32> = generate_random_modint32_vector(ring_degree);
    let poly = Polynomial {
        ring_degree,
        coeffs: rand_vec,
    };    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(poly.multiply_by_x());
        })
    });
}

pub fn bench_modint64(c: &mut Criterion, ring_degree: usize) {
    let name = format!("Add polynomials of degree {} with type {}", ring_degree,
                       std::any::type_name::<ModInteger64>());
    let rand_vec: Vec<ModInteger64> = generate_random_modint64_vector(ring_degree);
    let poly = Polynomial {
        ring_degree,
        coeffs: rand_vec,
    };
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(poly.multiply_by_x());
        })
    });
}

pub fn bench_f32_2048(c: &mut Criterion) {
    bench_float::<f32>(c, 2048);
}

pub fn bench_f32_4096(c: &mut Criterion) {
    bench_float::<f32>(c, 4096);
}

pub fn bench_f32_8192(c: &mut Criterion) {
    bench_float::<f32>(c, 8192);
}

pub fn bench_f32_16384(c: &mut Criterion) {
    bench_float::<f32>(c, 16384);
}

pub fn bench_f64_2048(c: &mut Criterion) {
    bench_float::<f64>(c, 2048);
}

pub fn bench_f64_4096(c: &mut Criterion) {
    bench_float::<f64>(c, 4096);
}

pub fn bench_f64_8192(c: &mut Criterion) {
    bench_float::<f64>(c, 8192);
}

pub fn bench_f64_16384(c: &mut Criterion) {
    bench_float::<f64>(c, 16384);
}

pub fn bench_complex_f32_2048(c: &mut Criterion) {
    bench_complex::<f32>(c, 2048);
}

pub fn bench_complex_f32_4096(c: &mut Criterion) {
    bench_complex::<f32>(c, 4096);
}

pub fn bench_complex_f32_8192(c: &mut Criterion) {
    bench_complex::<f32>(c, 8192);
}

pub fn bench_complex_f32_16384(c: &mut Criterion) {
    bench_complex::<f32>(c, 16384);
}

pub fn bench_complex_f64_2048(c: &mut Criterion) {
    bench_complex::<f64>(c, 2048);
}

pub fn bench_complex_f64_4096(c: &mut Criterion) {
    bench_complex::<f64>(c, 4096);
}

pub fn bench_complex_f64_8192(c: &mut Criterion) {
    bench_complex::<f64>(c, 8192);
}

pub fn bench_complex_f64_16384(c: &mut Criterion) {
    bench_complex::<f64>(c, 16384);
}

pub fn bench_modint32_2048(c: &mut Criterion) {
    bench_modint32(c, 2048);
}

pub fn bench_modint32_4096(c: &mut Criterion) {
    bench_modint32(c, 4096);
}

pub fn bench_modint32_8192(c: &mut Criterion) {
    bench_modint32(c, 8192);
}

pub fn bench_modint32_16384(c: &mut Criterion) {
    bench_modint32(c, 16384);
}

pub fn bench_modint64_2048(c: &mut Criterion) {
    bench_modint64(c, 2048);
}

pub fn bench_modint64_4096(c: &mut Criterion) {
    bench_modint64(c, 4096);
}

pub fn bench_modint64_8192(c: &mut Criterion) {
    bench_modint64(c, 8192);
}

pub fn bench_modint64_16384(c: &mut Criterion) {
    bench_modint64(c, 16384);
}