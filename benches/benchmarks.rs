use aoc2025::days::all_days;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::fs;

pub fn benchmark_days(c: &mut Criterion) {
    for (day_name, solve_fn) in all_days() {
        let input_path = format!("inputs/{}.txt", day_name);
        let input = fs::read_to_string(&input_path)
            .unwrap_or_else(|_| panic!("Failed to read input file for {}", day_name));

        c.bench_function(day_name, |b| {
            b.iter(|| {
                let _ = solve_fn(black_box(&input));
            })
        });
    }
}

criterion_group!(benches, benchmark_days);
criterion_main!(benches);
