use std::collections::HashMap;

type Coord = (usize, usize);

/// Solve returns (part1, part2) as strings.
pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1.to_string(), part2.to_string())
}

fn solve_part1(input: &str) -> String {
    let mut grid: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        let row: Vec<&str> = line.split("").filter(|c| !c.is_empty()).collect();
        grid.push(row);
    }

    let r = run_once(grid);

    r.to_string()
}

fn solve_part2(input: &str) -> String {
    let mut grid: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        let row: Vec<&str> = line.split("").filter(|c| !c.is_empty()).collect();
        grid.push(row);
    }

    let r = run_until_stable(&mut grid);

    r.to_string()
}

fn build_neighbour_cache(rows: usize, cols: usize) -> HashMap<Coord, Vec<Coord>> {
    let mut cache = HashMap::new();

    let directions = [
        (-1, -1), // upper-left
        (-1, 0),  // up
        (-1, 1),  // upper-right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // bottom-left
        (1, 0),   // bottom
        (1, 1),   // bottom-right
    ];

    for r in 0..rows {
        for c in 0..cols {
            let mut neighbours = Vec::new();

            for (dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nc >= 0 {
                    let (nr, nc) = (nr as usize, nc as usize);

                    if nr < rows && nc < cols {
                        neighbours.push((nr, nc));
                    }
                }
            }

            cache.insert((r, c), neighbours);
        }
    }

    cache
}

fn find_removable_cells(grid: &Vec<Vec<&str>>, cache: &HashMap<Coord, Vec<Coord>>) -> Vec<Coord> {
    let mut removable = Vec::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            // Only @ cells are candidates
            if *cell != "@" {
                continue;
            }

            // Count neighbour @ cells
            let mut count = 0;

            for &(nr, nc) in &cache[&(r, c)] {
                if grid[nr][nc] == "@" {
                    count += 1;
                }
            }

            if count < 4 {
                removable.push((r, c));
            }
        }
    }

    removable
}

fn run_once(grid: Vec<Vec<&str>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let cache = build_neighbour_cache(rows, cols);

    let removable = find_removable_cells(&grid, &cache);

    removable.len()
}

fn run_until_stable(grid: &mut Vec<Vec<&str>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let cache = build_neighbour_cache(rows, cols);

    let mut result = 0;

    loop {
        let removable = find_removable_cells(grid, &cache);

        if removable.is_empty() {
            return result;
        } else {
            result += removable.len()
        }

        // Remove them by replacing with "."
        for &(r, c) in &removable {
            grid[r][c] = ".";
        }
    }
}
