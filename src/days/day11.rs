use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn solve_part1(input: &str) -> String {
    let graph: HashMap<String, Vec<String>> = parse_input(input);
    let starting_node = "you".to_string();
    let ending_node = "out".to_string();
    let mut visited: HashMap<String, bool> = HashMap::new();

    let result = dfs(&graph, &starting_node, &ending_node, &mut visited);

    result.to_string()
}

fn solve_part2(input: &str) -> String {
    let graph: HashMap<String, Vec<String>> = parse_input(input);
    let starting_node = "svr".to_string();
    let ending_node = "out".to_string();
    let required_nodes = vec!["dac".to_string(), "fft".to_string()];
    let seen_required: HashSet<String> = HashSet::new();
    let mut memo: HashMap<(String, Vec<String>), usize> = HashMap::new();

    let result = dfs_with_required_nodes(
        &graph,
        &starting_node,
        &ending_node,
        &required_nodes,
        seen_required,
        &mut memo,
    );

    result.to_string()
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let node = parts.next().unwrap().to_string();
            let neighbors = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect();
            (node, neighbors)
        })
        .collect()
}

fn dfs(
    graph: &HashMap<String, Vec<String>>,
    current_node: &String,
    ending_node: &String,
    visited: &mut HashMap<String, bool>,
) -> usize {
    if current_node == ending_node {
        return 1;
    }

    visited.insert(current_node.clone(), true);

    let mut count = 0;

    if let Some(neighbors) = graph.get(current_node) {
        for neighbor in neighbors {
            if !visited.get(neighbor).unwrap_or(&false) {
                count += dfs(graph, neighbor, ending_node, visited);
            }
        }
    }

    visited.remove(current_node);
    count
}

fn dfs_with_required_nodes(
    graph: &HashMap<String, Vec<String>>,
    current_node: &String,
    ending_node: &String,
    required_nodes: &Vec<String>,
    seen_required: HashSet<String>,
    memo: &mut HashMap<(String, Vec<String>), usize>,
) -> usize {
    if current_node == ending_node {
        // only count this path if all required nodes have been seen
        return if seen_required.len() == required_nodes.len() {
            1
        } else {
            0
        };
    }

    // create memoization key
    let mut seen_keys: Vec<String> = seen_required.iter().cloned().collect();
    seen_keys.sort();
    let memo_key = (current_node.clone(), seen_keys);

    // return cached result if exists
    if let Some(&cached) = memo.get(&memo_key) {
        return cached;
    }

    // check if current node is a required node
    let mut new_seen = seen_required.clone();
    if required_nodes.contains(current_node) {
        new_seen.insert(current_node.clone());
    }

    let mut count = 0;
    if let Some(neighbors) = graph.get(current_node) {
        for neighbor in neighbors {
            if !seen_required.contains(neighbor) {
                count += dfs_with_required_nodes(
                    graph,
                    neighbor,
                    ending_node,
                    required_nodes,
                    new_seen.clone(),
                    memo,
                );
            }
        }
    }

    memo.insert(memo_key, count);

    count
}
