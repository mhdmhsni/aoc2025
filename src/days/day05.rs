/// Solve returns (part1, part2) as strings.
pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    let split_input: Vec<&str> = input.split("\n\n").collect();

    let (fresh_ranges, ingredients) = parse_input(&split_input);

    let mut found_fresh_ingredients = 0;

    for ingredient in &ingredients {
        for range in &fresh_ranges {
            let range_start = range.0;
            let range_end = range.1;

            if ingredient >= &range_start && ingredient <= &range_end {
                found_fresh_ingredients += 1;
                break;
            }
        }
    }

    found_fresh_ingredients.to_string()
}

fn solve_part2(input: &str) -> String {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let (mut fresh_ranges, _) = parse_input(&split_input);

    fresh_ranges.sort_by_key(|r| r.0);

    let mut total = 0;
    let mut current_start = fresh_ranges[0].0;
    let mut current_end = fresh_ranges[0].1;

    // merging ranges
    for &(start, end) in &fresh_ranges[1..] {
        if start <= current_end {
            // overlapping or touching
            if end > current_end {
                current_end = end;
            }
        } else {
            //non-overlapping
            total += current_end - current_start + 1;

            // make a new merged range
            current_start = start;
            current_end = end;
        }
    }

    total += current_end - current_start + 1;

    total.to_string()
}

fn parse_input(input: &Vec<&str>) -> (Vec<(i64, i64)>, Vec<i64>) {
    // parse ranges
    let ranges = input[0]
        .split("\n")
        .map(|range| {
            let (start, end) = range.split_once('-').expect("Invalid range format");
            (
                start.parse::<i64>().expect("Invalid number"),
                end.parse::<i64>().expect("Invalid number"),
            )
        })
        .collect();

    // parse ingredients
    let ingredients: Vec<i64> = input[1]
        .split("\n")
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    (ranges, ingredients)
}
