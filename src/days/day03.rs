/// Solve returns (part1, part2) as strings.
pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    const DESIRED_LENGTH: usize = 2;
    let mut sum = 0;
    for line in input.lines() {
        let result = joltage(line, DESIRED_LENGTH);
        match result {
            Some(n) => sum += n,
            None => panic!("No solution found"),
        }
    }

    sum.to_string()
}

fn solve_part2(input: &str) -> String {
    const DESIRED_LENGTH: usize = 12;
    let mut sum: u64 = 0;
    for line in input.lines() {
        let result = joltage(line, DESIRED_LENGTH);
        match result {
            Some(n) => sum += n,
            None => panic!("No solution found"),
        }
    }

    sum.to_string()
}

fn joltage(bank: &str, desired_length: usize) -> Option<u64> {
    let digits: Vec<u8> = bank
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8))
        .collect::<Option<Vec<_>>>()?;

    let n = digits.len();
    if n < desired_length {
        return None;
    }

    // We must remove exactly (n - K) digits to end with K digits.
    let mut to_remove = n - desired_length;
    let mut stack: Vec<u8> = Vec::with_capacity(n);

    for &d in &digits {
        // While we can remove and doing so would make the sequence lexicographically larger,
        // pop smaller top elements.
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }

    // If we still need to remove (monotonic sequence), drop from the end.
    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    // Now stack.len() >= K; take the first K elements
    let result_digits = &stack[..desired_length];

    // Convert to u64 with overflow checks
    let mut value: u64 = 0;
    for &d in result_digits {
        value = value.checked_mul(10)?.checked_add(d as u64)?;
    }

    Some(value)
}
