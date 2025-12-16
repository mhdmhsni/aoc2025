// ---------------- Solution ----------------
// Resources and algorithms used:
// - Disjoint Set (Union-Find) data structure
// - Kruskal's algorithm for Minimum Spanning Tree https://en.wikipedia.org/wiki/Kruskal%27s_algorithm
// - Minimum Spanning Tree is not directly computed, but the idea of connecting components using the smallest edges is similar.
// - https://en.wikipedia.org/wiki/Minimum_spanning_tree
// - Sorting edges by weight
// - Graph connectivity
// - Distance calculation in 3D space
// - Efficient component size tracking

pub fn solve(input: &str) -> (String, String) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn solve_part1(input: &str) -> String {
    let points = parse_points(input);
    let n = points.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((squared_distance(&points[i], &points[j]), i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut dsu = DisjointSet::new(n);

    for &(_, a, b) in edges.iter().take(1000) {
        dsu.union(a, b);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    (sizes[0] * sizes[1] * sizes[2]).to_string()
}

fn solve_part2(input: &str) -> String {
    let points = parse_points(input);
    let n = points.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((squared_distance(&points[i], &points[j]), i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut dsu = DisjointSet::new(n);
    let mut components = n;

    for &(_, a, b) in &edges {
        if dsu.union(a, b) {
            components -= 1;

            // This edge makes the graph fully connected
            if components == 1 {
                let result = points[a].x * points[b].x;
                return result.to_string();
            }
        }
    }

    unreachable!("Graph should become fully connected");
}

/* ---------------- Helpers ---------------- */

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut it = line.split(',');
            Point {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
                z: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn squared_distance(a: &Point, b: &Point) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

/* ---------------- Union-Find ---------------- */

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let mut counts = vec![0; self.parent.len()];
        for i in 0..self.parent.len() {
            let root = self.find(i);
            counts[root] += 1;
        }
        counts.into_iter().filter(|&c| c > 0).collect()
    }
}
