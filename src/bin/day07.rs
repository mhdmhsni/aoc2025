use aoc2025::days::day07;
use std::fs;

fn main() {
    let input_path = "inputs/day07.txt".to_string();
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| {
            eprintln!("Failed to read {}", input_path);
            std::process::exit(2);
        });

    let (p1, p2) = day07::solve(&input);
    println!("day07");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}
