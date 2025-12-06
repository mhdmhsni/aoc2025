use std::collections::HashMap;

/// Solve returns (part1, part2) as strings.
pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut problems_map: HashMap<usize, Vec<&str>> = HashMap::new();
    let mut grand_total: usize = 0;

    for line in lines {
        // TODO: matrix here as well??! (?
        let rows: Vec<&str> = line.split(" ").filter(|s| !s.is_empty()).collect();

        for (i, col) in rows.iter().enumerate() {
            problems_map.entry(i).or_insert_with(Vec::new).push(*col);
        }
    }

    for (_, col_values) in &problems_map {
        // separate operands and operators
        let (operands, operators): (Vec<&&str>, Vec<&&str>) = col_values
            .iter()
            .partition(|value| value.parse::<usize>().is_ok());

        for operator in operators {
            match *operator {
                "+" => {
                    let sum: usize = operands.iter().map(|op| op.parse::<usize>().unwrap()).sum();
                    grand_total += sum;
                }
                "*" => {
                    let product: usize = operands
                        .iter()
                        .map(|op| op.parse::<usize>().unwrap())
                        .product();
                    grand_total += product;
                }
                _ => {}
            }
        }
    }

    grand_total.to_string()
}

fn solve_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut matrix: Vec<Vec<&str>> = Vec::new();

    let mut grand_total: usize = 0;

    for line in lines {
        let rows: Vec<&str> = line.split("").collect();
        matrix.push(rows);
    }

    let transposed: Vec<Vec<&str>> = transpose(matrix);

    let problems_block: Vec<&[Vec<&str>]> = transposed
        .split(|col| col.iter().all(|&s| s.trim().is_empty()))
        .filter(|block| !block.is_empty())
        .collect();

    let trimmed_blocks: Vec<Vec<Vec<&str>>> = problems_block
        .iter()
        .map(|block| {
            block
                .iter()
                .map(|col| {
                    col.iter()
                        .filter(|&&s| !s.trim().is_empty())
                        .cloned()
                        .collect()
                })
                .collect()
        })
        .collect();

    for block in trimmed_blocks {
        let mut block_operator = String::new();
        let mut block_operands: Vec<usize> = Vec::new();

        for vec in block {
            // Check if all are numeric
            let all_numeric = vec.iter().all(|s| s.chars().all(|c| c.is_numeric()));

            if all_numeric {
                // Concat all
                let concat_numbers = vec.join("");
                block_operands.push(concat_numbers.parse::<usize>().unwrap());
            } else {
                // Partition
                let (numbers, operators): (Vec<&str>, Vec<&str>) =
                    vec.iter().partition(|s| s.chars().all(|c| c.is_numeric()));

                let concat_numbers = numbers.join("");

                block_operands.push(concat_numbers.parse::<usize>().unwrap());
                block_operator = operators[0].to_string(); // because we are assuming one operator per block
            }
        }

        match block_operator.as_str() {
            "+" => {
                let sum: usize = block_operands.iter().sum();
                grand_total += sum;
            }
            "*" => {
                let product: usize = block_operands.iter().product();
                grand_total += product;
            }
            _ => {}
        }
    }

    grand_total.to_string()
}

fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }

    (0..matrix[0].len())
        .map(|col| matrix.iter().map(|row| row[col].clone()).collect())
        .collect()
}
