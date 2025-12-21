use std::isize;

use advent_of_code_2025::*;

#[derive(Debug)]
struct RedTile {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Map {
    points: Vec<RedTile>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Map {
    fn new(points: Vec<RedTile>) -> Self {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for point in &points {
            min_x = std::cmp::min(point.x, min_x);
            max_x = std::cmp::max(point.x, max_x);
            min_y = std::cmp::min(point.y, min_y);
            max_y = std::cmp::min(point.y, max_y);
        }
        Self {
            points,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

impl RedTile {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn rectangle_area(&self, other: &RedTile) -> isize {
        let answer = ((other.x - self.x).abs() + 1) * ((other.y - self.y).abs() + 1);
        debug_println!(
            "Area of rectangle made from {:?} and {:?} is {answer}",
            self,
            other
        );
        answer
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

fn part_one(source: &Map) -> isize {
    let _t = Timer::start("Part One");
    let n = source.points.len();
    let sizes: Vec<isize> = (0..n)
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

    // #[rstest]
    // fn test_part_two_from_sample_data() {
    //     let source = parse_input("./data/day9_test");
    //     assert_eq!(part_two(&source), 50);
    // }
}
