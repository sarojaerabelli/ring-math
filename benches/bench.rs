use criterion::{criterion_group, criterion_main};

mod add;
mod multiply;
mod multiply_by_x;

criterion_group!(add_bench, add::bench_f32_2048,  add::bench_f32_4096,
                            add::bench_f32_8192,  add::bench_f32_16384,
                            add::bench_f64_2048,  add::bench_f64_4096,
                            add::bench_f64_8192,  add::bench_f64_16384,
                            add::bench_complex_f32_2048, add::bench_complex_f32_4096,
                            add::bench_complex_f32_8192, add::bench_complex_f32_16384,
                            add::bench_complex_f64_2048, add::bench_complex_f64_4096,
                            add::bench_complex_f64_8192, add::bench_complex_f64_16384,
                            add::bench_modint32_2048, add::bench_modint32_4096,
                            add::bench_modint32_8192, add::bench_modint32_16384,
                            add::bench_modint64_2048, add::bench_modint64_4096,
                            add::bench_modint64_8192, add::bench_modint64_16384);

criterion_group!(multiply_bench, multiply::bench_f32_2048,  multiply::bench_f32_4096,
                                 multiply::bench_f32_8192,  multiply::bench_f32_16384,
                                 multiply::bench_f64_2048,  multiply::bench_f64_4096,
                                 multiply::bench_f64_8192,  multiply::bench_f64_16384,
                                 multiply::bench_complex_f32_2048, multiply::bench_complex_f32_4096,
                                 multiply::bench_complex_f32_8192, multiply::bench_complex_f32_16384,
                                 multiply::bench_complex_f64_2048, multiply::bench_complex_f64_4096,
                                 multiply::bench_complex_f64_8192, multiply::bench_complex_f64_16384,
                                 multiply::bench_modint32_2048, multiply::bench_modint32_4096,
                                 multiply::bench_modint32_8192, multiply::bench_modint32_16384,
                                 multiply::bench_modint64_2048, multiply::bench_modint64_4096,
                                 multiply::bench_modint64_8192, multiply::bench_modint64_16384);

criterion_group!(multiply_by_x_bench, multiply_by_x::bench_f32_2048,  multiply_by_x::bench_f32_4096,
                                      multiply_by_x::bench_f32_8192,  multiply_by_x::bench_f32_16384,
                                      multiply_by_x::bench_f64_2048,  multiply_by_x::bench_f64_4096,
                                      multiply_by_x::bench_f64_8192,  multiply_by_x::bench_f64_16384,
                                      multiply_by_x::bench_complex_f32_2048, multiply_by_x::bench_complex_f32_4096,
                                      multiply_by_x::bench_complex_f32_8192, multiply_by_x::bench_complex_f32_16384,
                                      multiply_by_x::bench_complex_f64_2048, multiply_by_x::bench_complex_f64_4096,
                                      multiply_by_x::bench_complex_f64_8192, multiply_by_x::bench_complex_f64_16384,
                                      multiply_by_x::bench_modint32_2048, multiply_by_x::bench_modint32_4096,
                                      multiply_by_x::bench_modint32_8192, multiply_by_x::bench_modint32_16384,
                                      multiply_by_x::bench_modint64_2048, multiply_by_x::bench_modint64_4096,
                                      multiply_by_x::bench_modint64_8192, multiply_by_x::bench_modint64_16384);

criterion_main!(add_bench, multiply_bench, multiply_by_x_bench);