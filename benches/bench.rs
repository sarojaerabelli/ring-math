use criterion::{criterion_group, criterion_main};

mod add;
mod multiply;
mod multiply_by_x;
mod matrix_vector_multiply;

criterion_group!(add_bench, add::bench_f32_2048,  add::bench_f32_4096,
                            add::bench_f64_2048,  add::bench_f64_4096,
                            add::bench_complex_f32_2048, add::bench_complex_f32_4096,
                            add::bench_complex_f64_2048, add::bench_complex_f64_4096,
                            add::bench_modint32_2048, add::bench_modint32_4096,
                            add::bench_modint64_2048, add::bench_modint64_4096);

criterion_group!(multiply_bench, multiply::bench_f32_2048,
                                 multiply::bench_f64_2048,
                                 multiply::bench_complex_f32_2048,
                                 multiply::bench_complex_f64_2048,
                                 multiply::bench_modint32_2048,
                                 multiply::bench_modint64_2048);

criterion_group!(multiply_by_x_bench, multiply_by_x::bench_f32_2048,  multiply_by_x::bench_f32_4096,
                                      multiply_by_x::bench_f64_2048,  multiply_by_x::bench_f64_4096,
                                      multiply_by_x::bench_complex_f32_2048, multiply_by_x::bench_complex_f32_4096,
                                      multiply_by_x::bench_complex_f64_2048, multiply_by_x::bench_complex_f64_4096,
                                      multiply_by_x::bench_modint32_2048, multiply_by_x::bench_modint32_4096,
                                      multiply_by_x::bench_modint64_2048, multiply_by_x::bench_modint64_4096);

criterion_group!(matrix_vector_multiply_bench, matrix_vector_multiply::bench_f32_degree_100_rows_10_cols_10,
                                               matrix_vector_multiply::bench_f64_degree_100_rows_10_cols_10,
                                               matrix_vector_multiply::bench_complex_f32_degree_100_rows_10_cols_10,
                                               matrix_vector_multiply::bench_complex_f64_degree_100_rows_10_cols_10,
                                               matrix_vector_multiply::bench_modint32_degree_100_rows_10_cols_10,
                                               matrix_vector_multiply::bench_modint64_degree_100_rows_10_cols_10);

criterion_main!(add_bench, multiply_bench, multiply_by_x_bench, matrix_vector_multiply_bench);