use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt;

use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;

use advent_of_code_2025::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct RedTile {
    x: usize,
    y: usize,
}

impl RedTile {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn rectangle_area(&self, other: &RedTile) -> usize {
        let answer = (max(other.x, self.x) - min(other.x, self.x) + 1)
            * (max(other.y, self.y) - min(other.y, self.y) + 1);
        debug_println!(
            "Area of rectangle made from {:?} and {:?} is {answer}",
            self,
            other
        );
        answer
    }
}

#[derive(Debug, Clone)]
struct Map {
    points: Vec<RedTile>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Map {
    fn new(points: Vec<RedTile>) -> Self {
        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let mut min_y = usize::MAX;
        let mut max_y = usize::MIN;
        for point in &points {
            min_x = min(point.x, min_x);
            max_x = max(point.x, max_x);
            min_y = min(point.y, min_y);
            max_y = max(point.y, max_y);
        }
        debug_println!(
            "Min x: {}\tMin y: {}\tMax x: {}\tMax y: {}",
            min_x,
            min_y,
            max_x,
            max_y
        );
        Self {
            points,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Do not use for large maps!  This eats up ram!
        // Helper to get indices
        let get_idx = |x: usize, y: usize| (x - self.min_x, y - self.min_y);
        let mut temp_output =
            vec![vec![false; (self.max_y - self.min_y) + 1]; (self.max_x - self.min_x) + 1];
        for point in self.points.iter().progress() {
            let (x1, y1) = get_idx(point.x, point.y);
            temp_output[x1][y1] = true;
        }
        // This is where it gets really nasty
        for line in temp_output.iter().progress() {
            let outstring: String = line
                .iter()
                .map(|x| match x {
                    true => '#',
                    false => '.',
                })
                .collect();
            write!(f, "{outstring}")?
        }
        Ok(())
    }
}

fn parse_input(filename: &str) -> Map {
    let _t = Timer::start(format!("Parsing file: {filename}"));
    Map::new(
        read_file(filename)
            .iter()
            .map(|l| {
                let mut iter = l.split(',');
                let x = iter.next().unwrap().parse().unwrap();
                let y = iter.next().unwrap().parse().unwrap();
                RedTile::new(x, y)
            })
            .collect(),
    )
}

fn make_bitmap(source: &Map) -> Vec<Vec<bool>> {
    let _t = Timer::start("Making Bitmap");

    // Trying to flood fill.  Not sure this is right?
    // Create a padded bitmap (padding of 1 on all sides)
    let height = (source.max_x - source.min_x + 1) + 2;
    let width = (source.max_y - source.min_y + 1) + 2;

    // false = potential outside/unknown, true = wall
    let mut is_wall = vec![vec![false; width]; height];

    // Helper to get indices with the +1 padding
    let get_idx = |x: usize, y: usize| (x - source.min_x + 1, y - source.min_y + 1);

    // Fill in the boundary edges
    println!("Filling in the boundary edges");
    let n = source.points.len();
    for i in 0..n {
        let a = source.points[i];
        let b = source.points[(i + 1) % n]; // Wraps to start automatically

        let (x1, y1) = get_idx(a.x, a.y);
        let (x2, y2) = get_idx(b.x, b.y);

        for x in is_wall
            .iter_mut()
            .take(std::cmp::max(x1, x2) + 1)
            .skip(std::cmp::min(x1, x2))
        {
            for y in x
                .iter_mut()
                .take(std::cmp::max(y1, y2) + 1)
                .skip(std::cmp::min(y1, y2))
            {
                *y = true;
            }
        }
    }

    // Flood Fill from (0,0) to find all "Outside" tiles
    // Creating a grid to track known outside points
    let mut is_outside = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();

    queue.push_back((0, 0));
    is_outside[0][0] = true;

    println!("Flood fill");
    while let Some((x, y)) = queue.pop_front() {
        if queue.len() % 1000 == 0 {
            println!("Remaining {}", queue.len());
        }
        debug_println!("{x},{y}");
        // Check 4 neighbors
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                // If it's not a wall and we haven't visited it yet, it's outside?
                if !is_wall[nx][ny] && !is_outside[nx][ny] {
                    is_outside[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    // Inside is everything that is NOT "outside"
    // Strip the padding back off during this step
    println!("Producing final bitmap");
    let mut final_bitmap = vec![vec![false; width - 2]; height - 2];
    for x in (1..(height - 1)).progress() {
        for y in 1..(width - 1) {
            if !is_outside[x][y] {
                final_bitmap[x - 1][y - 1] = true;
            }
        }
    }

    final_bitmap
}

fn part_two(source: &Map) -> usize {
    let _t = Timer::start("Part Two");
    let bitmap = make_bitmap(source);
    debug_println!("{:?}", bitmap);
    let n = source.points.len();
    let get_idx = |x: usize, y: usize| (x - source.min_x, y - source.min_y);
    let sizes: Vec<usize> = (0..n)
        .into_par_iter()
        .progress()
        .flat_map(|f| {
            (f + 1..n)
                .filter_map(|g| {
                    let min_x = min(source.points[f].x, source.points[g].x);
                    let max_x = max(source.points[f].x, source.points[g].x);
                    let min_y = min(source.points[f].y, source.points[g].y);
                    let max_y = max(source.points[f].y, source.points[g].y);
                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            let (off_x, off_y) = get_idx(x, y);
                            if !bitmap[off_x][off_y] {
                                return None;
                            }
                        }
                    }
                    Some(source.points[f].rectangle_area(&source.points[g]))
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let answer = sizes.iter().max().unwrap();
    println!("Part Two Result: {answer}");
    *answer
}

fn part_one(source: &Map) -> usize {
    let _t = Timer::start("Part One");
    let n = source.points.len();
    let sizes: Vec<usize> = (0..n)
        .flat_map(|f| {
            (f + 1..n)
                .map(|g| source.points[f].rectangle_area(&source.points[g]))
                .collect::<Vec<_>>()
        })
        .collect();
    debug_println!("{:?}", sizes);
    let answer = sizes.iter().max().unwrap();
    println!("Part One Result: {answer}");
    *answer
}

fn main() {
    let _t = Timer::start("Day 9");
    let data = parse_input("./data/day9.txt");
    part_one(&data);
    part_two(&data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one_from_sample_data() {
        let source = parse_input("./data/day9_test");
        assert_eq!(part_one(&source), 50);
    }

    #[rstest]
    fn test_part_two_from_sample_data() {
        let source = parse_input("./data/day9_test");
        assert_eq!(part_two(&source), 24);
    }
}
