use std::{
    collections::{HashMap, HashSet},
    f64,
};

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

    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)) as f64)
            .sqrt()
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

fn part_one(source: &Vec<Point>) {
    // This is a horribly inefficient approach
    let mut seen: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut connections: HashMap<Point, HashSet<Point>> = HashMap::new();
    for first in source {
        let mut min_distance = f64::MAX;
        let mut min_other = Point::new(isize::MAX, isize::MAX, isize::MAX);
        for second in source {
            if first == second {
                continue;
            }
            let distance = first.distance(second);
            if min_distance > distance {
                min_distance = distance;
                min_other = second.clone();
            }
        }
        connections
            .entry(*first)
            .or_insert(HashSet::new())
            .insert(min_other);
    }
    println!("After processing {:?}", connections);
}

fn main() {
    let data = parse_input("./data/day8_test");
    part_one(&data);
}
