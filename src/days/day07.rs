/// Solve returns (part1, part2) as strings.

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
                _ => {} // it's either out of bounds or an invalid cell
            }
        }

        active_beams = temp_beams.clone();
        temp_beams.clear();
        split_count += new_splits;
    }

    split_count.to_string()
}

fn solve_part2(input: &str) -> String {
    String::new()
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

fn split(mut temp_beams: &mut Vec<Coord>, processing_item: Coord) {
    let left_beam = (processing_item.0, processing_item.1 - 1);
    let right_beam = (processing_item.0, processing_item.1 + 1);
    // left beam
    push_if_not_exists(&mut temp_beams, left_beam);
    //right beam
    push_if_not_exists(&mut temp_beams, right_beam);
}

fn push_if_not_exists(vec: &mut Vec<Coord>, coord: Coord) {
    if !vec.contains(&coord) {
        vec.push(coord);
    }
}
