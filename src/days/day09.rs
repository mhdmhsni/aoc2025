use std::collections::HashMap;

/// Solve returns (part1, part2) as strings.
pub fn solve(input: &str) -> (String, String) {
    // TODO: implement
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut points: Vec<(usize, usize)> = Vec::new();
    for line in lines {
        let (a, b) = line.split_once(",").unwrap();
        let x: usize = a.parse().unwrap();
        let y: usize = b.parse().unwrap();
        points.push((x, y));
    }

    let sorted_points = {
        let mut sp = points.clone();
        sp.sort();
        sp
    };

    let num_points = sorted_points.len();
    let mut area_map: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();

    for idx in 0..num_points {
        let point1 = sorted_points[idx];
        let mut point2: (usize, usize) = (0, 0);
        for jdx in (idx + 1)..num_points {
            point2 = sorted_points[jdx];
            calculate_rectangular_area(point1, point2, &mut area_map);
        }
    }

    area_map.values().max().unwrap().to_string()
}

fn solve_part2(input: &str) -> String {
    String::new()
}

fn calculate_rectangular_area(
    point1: (usize, usize),
    point2: (usize, usize),
    area_map: &mut HashMap<((usize, usize), (usize, usize)), usize>,
) -> usize {
    let width = (point1.0 as isize - point2.0 as isize).abs() as usize + 1;
    let height = (point2.1 as isize - point1.1 as isize).abs() as usize + 1;
    let area = width * height;

    if area_map.contains_key(&(point1, point2)) {
        return *area_map.get(&(point1, point2)).unwrap();
    } else {
        area_map.insert((point1, point2), area);
        area_map.insert((point2, point1), area);
        area
    }
}
