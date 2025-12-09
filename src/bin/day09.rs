use aoc2025::days::day09;
use std::fs;

fn main() {
    let input_path = "inputs/day09.txt".to_string();
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| {
            eprintln!("Failed to read {}", input_path);
            std::process::exit(2);
        });

    let (p1, p2) = day09::solve(&input);
    println!("day09");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}
