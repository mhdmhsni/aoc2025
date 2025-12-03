use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin newday <day>");
        std::process::exit(1);
    }

    let day = &args[1];
    let day_num: u32 = day.parse().expect("Day must be a number");
    let day_str = format!("day{:02}", day_num);

    println!("Generating {day_str}...");

    // 1) Create inputs/dayXX.txt
    fs::create_dir_all("inputs").unwrap();
    let input_path = format!("inputs/{day_str}.txt");
    if !Path::new(&input_path).exists() {
        fs::write(&input_path, "").unwrap();
        println!("  created {input_path}");
    } else {
        println!("  {input_path} already exists");
    }

    // 2) Create src/days/dayXX.rs
    fs::create_dir_all("src/days").unwrap();
    let day_file = format!("src/days/{day_str}.rs");
    if !Path::new(&day_file).exists() {
        fs::write(&day_file, TEMPLATE_DAY.replace("DAY", &day_str)).unwrap();
        println!("  created {day_file}");
    } else {
        println!("  {day_file} already exists");
    }

    // 3) Update src/days/mod.rs: add `pub mod dayXX;` and add to all_days() vector
    let mod_path = "src/days/mod.rs";
    let mut mod_file = fs::read_to_string(mod_path).expect("src/days/mod.rs must exist");
    let mod_line = format!("pub mod {day_str};");
    if !mod_file.contains(&mod_line) {
        // insert mod declaration near top (append at bottom of mod declarations)
        // Heuristic: append before the all_days() fn
        if let Some(pos) = mod_file.find("/// Returns a Vec of (module_name") {
            // put declaration before the doc comment
            let (head, tail) = mod_file.split_at(pos);
            mod_file = format!("{}{}\n{}", head, mod_line, tail);
        } else {
            // fallback append
            mod_file.push_str(&format!("\n{mod_line}\n"));
        }

        // also add to the all_days() vector: append a line in the vec![] body
        // look for the vector literal start "vec!["
        if let Some(start) = mod_file.find("vec![") {
            // naive insertion: find the closing bracket of vec! (first occurrence of "];" after start)
            if let Some(close) = mod_file[start..].find("];") {
                let insert_pos = start + close;
                // build entry line e.g. ("day05", day05::solve as fn(&str) -> (String, String)),
                let entry = format!(
                    "    (\"{day_str}\", {day_str}::solve as fn(&str) -> (String, String)),\n"
                );
                mod_file.insert_str(insert_pos, &entry);
            }
        }

        fs::write(mod_path, mod_file).unwrap();
        println!("  updated {mod_path}");
    } else {
        println!("  {mod_path} already contains mod {day_str}");
    }

    // 4) Create src/bin/dayXX.rs
    let bin_file = format!("src/bin/{day_str}.rs");
    if !Path::new(&bin_file).exists() {
        fs::write(&bin_file, TEMPLATE_BIN.replace("DAY", &day_str)).unwrap();
        println!("  created {bin_file}");
    } else {
        println!("  {bin_file} already exists");
    }

    println!("Done! Created solution skeleton for {day_str}");
}

static TEMPLATE_DAY: &str = r#"/// Solve returns (part1, part2) as strings.
pub fn solve(input: &str) -> (String, String) {
    // TODO: implement
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1.to_string(), part2.to_string())
}

fn solve_part1(input: &str) -> String {
    String::new()
}

fn solve_part2(input: &str) -> String {
    String::new()
}
"#;

static TEMPLATE_BIN: &str = r#"use aoc2025::days::DAY;
use std::fs;

fn main() {
    let input_path = "inputs/DAY.txt".to_string();
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| {
            eprintln!("Failed to read {}", input_path);
            std::process::exit(2);
        });

    let (p1, p2) = DAY::solve(&input);
    println!("DAY");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}
"#;
