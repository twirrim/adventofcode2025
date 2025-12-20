use std::collections::HashMap;

use advent_of_code_2025::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn dist_sq(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

fn parse_input(filename: &str) -> Vec<Point> {
    let _t = Timer::start(format!("Parsing file {filename}"));
    read_file(filename)
        .iter()
        .map(|f| {
            let mut iter = f.split(',').map(|s| s.trim().parse::<isize>().unwrap());
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            let z = iter.next().unwrap();
            Point::new(x, y, z)
        })
        .collect()
}

#[derive(Debug)]
struct DisjoinSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_groups: usize,
}

impl DisjoinSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_groups: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        debug_println!("Does {} = {}", self.parent[i], i);
        if self.parent[i] != i {
            // path compression
            debug_println!("No, recursing to find {}'s parent", self.parent[i]);
            self.parent[i] = self.find(self.parent[i]);
        }
        debug_println!("Yes, returning {}", self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            // merge smallest tree into biggest tree
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.num_groups -= 1;
        }
    }

    // Returns a mapping of group id to count
    fn get_group_sizes(&mut self) -> HashMap<usize, usize> {
        let mut counts = HashMap::new();
        // Call find on everyone to ensure path compression has occurred,
        // and find self referential parents
        for i in 0..self.parent.len() {
            let root = self.find(i);
            counts.insert(root, self.size[root]);
        }
        counts
    }
}

fn part_two(source: &[Point]) -> isize {
    let _t = Timer::start("Part Two");
    let n = source.len();
    assert!(n >= 2, "Something is wrong with the input data");

    let mut edges: Vec<(i64, usize, usize)> = (0..n)
        .flat_map(|i| {
            (i + 1..n).map(move |j| {
                let d = source[i].dist_sq(&source[j]);
                (d, i, j)
            })
        })
        .collect();

    // I think we need to sort here?
    edges.sort_unstable_by_key(|a| a.0);

    // Build the disjoint
    let mut dsu = DisjoinSet::new(n);
    let mut count = 0;
    let mut answer: isize = 0;
    for (_, i, j) in edges {
        count += 1;
        debug_println!("#### Adding union between {i} and {j}");
        dsu.union(i, j);
        if dsu.num_groups == 1 {
            // We can quit!
            debug_println!("Stopped after {count} connections");
            debug_println!(
                "Final connection was between {:?} and {:?}",
                source[i],
                source[j]
            );
            answer = source[i].x * source[j].x;
            break;
        }
    }

    println!("Part Two Result: {answer}");
    answer
}

fn part_one(source: &[Point], pairs_to_connect: usize) -> usize {
    let _t = Timer::start("Part One");
    let n = source.len();
    assert!(n >= 2, "Something is wrong with the input data");

    let mut edges: Vec<(i64, usize, usize)> = (0..n)
        .flat_map(|i| {
            (i + 1..n).map(move |j| {
                let d = source[i].dist_sq(&source[j]);
                (d, i, j)
            })
        })
        .collect();
    debug_println!("{:?}", edges);

    // Don't do a full sort, QuickSelect will help us here.  We're picking the nth smallest by key.
    if pairs_to_connect < edges.len() {
        edges.select_nth_unstable_by_key(pairs_to_connect, |a| a.0);
        edges.truncate(pairs_to_connect);
    }

    // Build the disjoint
    let mut dsu = DisjoinSet::new(n);
    for (_, i, j) in edges {
        dsu.union(i, j);
    }

    // Then get the result
    let sizes = dsu.get_group_sizes();
    let mut biggest_sizes: Vec<&usize> = sizes.values().collect();
    biggest_sizes.sort();

    debug_println!("Found {} distinct groups.", sizes.len());
    debug_println!("Group sizes: {:?}", sizes.values().collect::<Vec<_>>());
    debug_println!("Sorted group: {:?}", biggest_sizes);
    // take the three largest values, multiply them together
    let mut answer = 1;
    (0..3).for_each(|_| answer *= biggest_sizes.pop().unwrap());
    println!("Part One Result: {answer}");

    answer
}

fn main() {
    let _t = Timer::start("Day 8");
    let data = parse_input("./data/day8.txt");
    part_one(&data, 1000);
    part_two(&data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one_from_sample_data() {
        let source = parse_input("./data/day8_test");
        assert_eq!(part_one(&source, 10), 40);
    }

    #[rstest]
    fn test_part_two_from_sample_data() {
        let source = parse_input("./data/day8_test");
        assert_eq!(part_two(&source), 25272);
    }
}
