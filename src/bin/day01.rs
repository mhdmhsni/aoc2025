use aoc2025::days::day01;
use std::fs;

fn main() {
    let input_path = "inputs/day01.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let (part1, part2) = day01::solve(&input);
    println!("Day 01 - Part 1: {}", part1);
    println!("Day 01 - Part 2: {}", part2);
}
