type Coord = (usize, usize);

pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split("").filter(|s| !s.is_empty()).collect())
        .collect();

    let (starting_pos, _) = navigate_grid(&grid);
    count_splits(&grid, starting_pos).to_string()
}

fn solve_part2(input: &str) -> String {
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split("").filter(|s| !s.is_empty()).collect())
        .collect();

    let (starting_pos, _) = navigate_grid(&grid);
    let mut memo = vec![vec![0i64; grid[0].len()]; grid.len()];

    count_timelines(&grid, starting_pos.0, starting_pos.1, &mut memo).to_string()
}

fn count_splits(grid: &Vec<Vec<&str>>, starting_pos: Coord) -> usize {
    let mut active_beams: Vec<Coord> = vec![starting_pos];
    let mut temp_beams: Vec<Coord> = Vec::new();
    let mut split_count = 0;

    for _ in 1..grid.len() - 1 {
        let mut new_splits = 0;
        for beam in active_beams.iter() {
            let processing_item = (beam.0 + 1, beam.1);
            match grid[processing_item.0][processing_item.1] {
                "." => {
                    push_if_not_exists(&mut temp_beams, processing_item);
                }
                "^" => {
                    new_splits += 1;
                    split(&mut temp_beams, processing_item);
                }
                _ => {}
            }
        }
        active_beams = temp_beams.clone();
        temp_beams.clear();
        split_count += new_splits;
    }

    split_count
}

fn count_timelines(grid: &Vec<Vec<&str>>, row: usize, col: usize, memo: &mut Vec<Vec<i64>>) -> i64 {
    if col >= grid[0].len() {
        return 0;
    }

    if row >= grid.len() {
        return 1;
    }

    if memo[row][col] > 0 {
        return memo[row][col];
    }

    let timelines = match grid[row][col] {
        "^" => {
            let left = if col > 0 {
                count_timelines(grid, row, col - 1, memo)
            } else {
                0
            };
            let right = count_timelines(grid, row, col + 1, memo);
            left + right
        }
        _ => count_timelines(grid, row + 1, col, memo),
    };

    memo[row][col] = timelines;
    timelines
}

fn navigate_grid(grid: &Vec<Vec<&str>>) -> (Coord, Vec<Coord>) {
    let mut starting_pos: Coord = (0, 0);
    let mut splitters: Vec<Coord> = Vec::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            match *cell {
                "S" => starting_pos = (row_idx + 1, col_idx),
                "^" => splitters.push((row_idx, col_idx)),
                _ => {}
            }
        }
    }

    (starting_pos, splitters)
}

fn split(temp_beams: &mut Vec<Coord>, processing_item: Coord) {
    let left_beam = (processing_item.0, processing_item.1 - 1);
    let right_beam = (processing_item.0, processing_item.1 + 1);

    push_if_not_exists(temp_beams, left_beam);
    push_if_not_exists(temp_beams, right_beam);
}

fn push_if_not_exists(vec: &mut Vec<Coord>, coord: Coord) {
    if !vec.contains(&coord) {
        vec.push(coord);
    }
}
