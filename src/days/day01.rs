pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    let mut value = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let (dir, steps) = parse_instruction(line);
        value = apply_instruction(value, dir, steps);

        if value == 0 {
            zero_count += 1;
        }
    }

    zero_count.to_string()
}

fn solve_part2(input: &str) -> String {
    let mut value = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let (dir, steps) = parse_instruction(line);
        let crosses = check_zero_crossing(value, dir, steps);
        value = apply_instruction(value, dir, steps);

        zeroes += crosses;
    }

    zeroes.to_string()
}

fn parse_instruction(s: &str) -> (char, i32) {
    let dir = s.chars().next().unwrap();
    let steps = s[1..].parse::<i32>().unwrap();
    (dir, steps)
}

fn apply_instruction(start: i32, dir: char, steps: i32) -> i32 {
    const LOCK_SIZE: i32 = 100;

    match dir {
        'R' => (start + steps).rem_euclid(LOCK_SIZE),
        'L' => (start - steps).rem_euclid(LOCK_SIZE),
        _ => panic!("Unknown direction: {}", dir),
    }
}

fn check_zero_crossing(start: i32, dir: char, steps: i32) -> i32 {
    const LOCK_SIZE: i32 = 100;

    // Calculate where we end up before wrapping
    let end = match dir {
        'L' => start - steps,
        'R' => start + steps,
        _ => panic!("Unknown direction: {}", dir),
    };

    // Count how many times we cross 0
    let zeros = match dir {
        'L' => (start - 1).div_euclid(LOCK_SIZE) - (end - 1).div_euclid(LOCK_SIZE),
        'R' => end.div_euclid(LOCK_SIZE) - start.div_euclid(LOCK_SIZE),
        _ => panic!("Unknown direction: {}", dir),
    };

    zeros
}
