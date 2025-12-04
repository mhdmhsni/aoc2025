/// Solve returns (part1, part2) as strings.
pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    let ranges = input.split(",");
    let mut result: i64 = 0;

    for range in ranges {
        let (from_str, to_str) = range.split_once('-').unwrap();
        let (from, to) = (
            from_str.parse::<i64>().unwrap(),
            to_str.parse::<i64>().unwrap(),
        );

        for id in from..=to {
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            } // we don't care about ids with odd length
            let (left_str, right_str) = id_str.split_at(id_str.len() / 2); // split from the middle
            let (left, right) = (
                left_str.parse::<i64>().unwrap(),
                right_str.parse::<i64>().unwrap(),
            );
            if left == right {
                result += id;
            }
        }
    }

    result.to_string()
}

fn solve_part2(input: &str) -> String {
    let ranges = input.split(",");
    let mut result: i64 = 0;

    // TODO: this is brute forcing and very slow, find a smarter solution
    for range in ranges {
        let (from_str, to_str) = range.split_once('-').unwrap();
        let (from, to) = (
            from_str.parse::<i64>().unwrap(),
            to_str.parse::<i64>().unwrap(),
        );

        for id in from..=to {
            let id_str = id.to_string();
            let len = id_str.len();
            let middle = len / 2;
            for portion in 1..=middle {
                // we don't need to iterate over the entire string, until the middle is enough
                let seq = split_by_size(&id_str, portion);
                if is_all_same(&seq) {
                    result += id;
                    break;
                }
            }
        }
    }

    result.to_string()
}

fn split_by_size(s: &str, size: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(size)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn is_all_same<T: PartialEq>(vec: &[T]) -> bool {
    if vec.is_empty() {
        return true;
    }
    let first = &vec[0];
    vec.iter().all(|item| item == first)
}
