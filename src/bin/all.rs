use aoc2025::days::all_days;
use std::fs;

fn main() {
    for (day_name, solve_fn) in all_days() {
        let input_path = format!("inputs/{}.txt", day_name);
        let input = match fs::read_to_string(&input_path) {
            Ok(content) => content,
            Err(_) => panic!("Failed to read input file for {}", day_name),
        };
        let (part1, part2) = solve_fn(&input);
        println!("{} - Part 1: {}", day_name, part1);
        println!("{} - Part 2: {}", day_name, part2);
        println!()
    }
}
