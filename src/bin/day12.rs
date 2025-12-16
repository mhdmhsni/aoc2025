use aoc2025::days::day12;
use std::fs;

fn main() {
    let input_path = "inputs/day12.txt".to_string();
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| {
            eprintln!("Failed to read {}", input_path);
            std::process::exit(2);
        });

    let (p1, p2) = day12::solve(&input);
    println!("day12");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}
