use criterion::{black_box, Criterion};
use zama_math::polynomial::Polynomial;
use utilities::generate_random_float_vector;

pub fn bench<T>(c: &mut Criterion, ring_degree: usize) {
    let name = format!("Multiply polynomials of degree {} by x with type {}", ring_degree, std::any::type_name::<T>());
    let rand_vec: Vec<f64> = generate_random_float_vector::<f64>(ring_degree);
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
    bench::<f32>(c, 2048);
}

pub fn bench_f32_4096(c: &mut Criterion) {
    bench::<f32>(c, 4096);
}

pub fn bench_f64_2048(c: &mut Criterion) {
    bench::<f64>(c, 2048);
}

pub fn bench_f64_4096(c: &mut Criterion) {
    bench::<f64>(c, 4096);
}
