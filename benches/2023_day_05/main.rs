use criterion::{Criterion, criterion_group, criterion_main};

const INPUT: &str  = include_str!("input.txt");

use advent_of_code_rs::year_2023::day_05::solve_part_two;


pub fn criterion_benchmarK(c: &mut Criterion) {
    c.bench_function("2023 05 part 2", |b| b.iter(|| solve_part_two(INPUT)));
}

criterion_group!(benches, criterion_benchmarK);
criterion_main!(benches);