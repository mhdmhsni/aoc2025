use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Schematic {
    lights: String,
    buttons: Vec<Vec<i32>>,
    joltage: Vec<i32>,
}

/// Solve returns (part1, part2) as strings.
pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    let lines = input.lines();
    let mut data: Vec<Schematic> = Vec::new();
    for line in lines.clone() {
        data.push(parse_input(line));
    }
    let results: usize = run_schematics(&data);

    results.to_string()
}

fn solve_part2(input: &str) -> String {
    String::new()
}

fn parse_input(line: &str) -> Schematic {
    let re: Regex = Regex::new(r"^[\[({](.*)[\])}]$").unwrap();
    let mut lights: String = String::new();
    let mut buttons: Vec<Vec<i32>> = vec![];
    let mut joltage: Vec<i32> = vec![];
    let splitted_by_space: Vec<&str> = line.split(' ').collect();

    for item in splitted_by_space {
        if item.starts_with("[") {
            // It's a light
            let replaced = re.replace(item, "$1");
            lights.push_str(&replaced);
        } else if item.starts_with("(") {
            // It's a button
            let replaced = re.replace(item, "$1");
            let nums: Vec<i32> = replaced
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            buttons.push(nums);
        } else if item.starts_with("{") {
            // It's a joltage
            let replaced = re.replace(item, "$1");
            let nums: Vec<i32> = replaced
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            joltage.extend(nums);
        }
    }

    Schematic {
        lights,
        buttons,
        joltage,
    }
}

fn run_schematics(data: &Vec<Schematic>) -> usize {
    data.iter()
        .map(|schematic| solve_schematic(schematic))
        .sum()
}

fn solve_schematic(schematic: &Schematic) -> usize {
    let target = &schematic.lights;
    let buttons = &schematic.buttons;
    bfs_solve(target, buttons)
}

fn bfs_solve(target: &str, buttons: &Vec<Vec<i32>>) -> usize {
    let initial_state = ".".repeat(target.len());

    if initial_state == target {
        return 0;
    }

    let mut queue = create_queue(initial_state.clone());
    let mut visited = create_visited_set(initial_state);

    search_for_solution(&mut queue, &mut visited, target, buttons)
}

fn create_queue(initial_state: String) -> VecDeque<(String, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back((initial_state, 0));
    queue
}

fn create_visited_set(initial_state: String) -> HashSet<String> {
    let mut visited = HashSet::new();
    visited.insert(initial_state);
    visited
}

fn search_for_solution(
    queue: &mut VecDeque<(String, usize)>,
    visited: &mut HashSet<String>,
    target: &str,
    buttons: &Vec<Vec<i32>>,
) -> usize {
    while let Some((state, presses)) = queue.pop_front() {
        if let Some(result) = explore_state(&state, presses, queue, visited, target, buttons) {
            return result;
        }
    }
    usize::MAX
}

fn explore_state(
    state: &str,
    presses: usize,
    queue: &mut VecDeque<(String, usize)>,
    visited: &mut HashSet<String>,
    target: &str,
    buttons: &Vec<Vec<i32>>,
) -> Option<usize> {
    for button in buttons {
        let new_state = toggle_lights(state, button);

        if &new_state == target {
            return Some(presses + 1);
        }

        enqueue_if_new(new_state, presses + 1, queue, visited);
    }
    None
}

fn toggle_lights(current_lights: &str, button: &Vec<i32>) -> String {
    let mut new_light: String = String::from(current_lights);
    for &index in button {
        if let Some(ch) = new_light.chars().nth(index as usize) {
            let toggled_ch = if ch == '.' {
                '#'
            } else if ch == '#' {
                '.'
            } else {
                ch
            };
            new_light.replace_range(index as usize..=index as usize, &toggled_ch.to_string());
        }
    }
    new_light
}

fn enqueue_if_new(
    state: String,
    presses: usize,
    queue: &mut VecDeque<(String, usize)>,
    visited: &mut HashSet<String>,
) {
    if visited.insert(state.clone()) {
        queue.push_back((state, presses));
    }
}
