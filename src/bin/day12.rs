use std::sync::LazyLock;

use regex::{Match, Regex};

use advent_of_code_2025::*;

// Made it all the way to day12 before I broke out a regex, and LazyLock.
const REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?<p_index>^\d+):|(?<shape>[#.]{3})|(?<dimensions>^\d+x\d+:)(?<requirements>.+)")
        .unwrap()
});

#[derive(Debug, PartialEq)]
struct Present {
    index: usize,
    shapes: Vec<[u8; 3]>,
}

impl Present {
    fn new(index: usize, shape: &[String; 3]) -> Self {
        let mut shape = shape.to_owned();
        let mut shapes = vec![];
        for _rotate in 0..4 {
            for _flip in 0..2 {
                let mut new_shape = [0u8; 3];
                for r in 0..3 {
                    let row = shape[r].chars().collect::<Vec<_>>();
                    for i in 0..3 {
                        match row[i] {
                            '.' => (),                         // don't need to do anything with 0s.
                            '#' => new_shape[r] += 1 << 2 - i, // 2 - i to reverse the ordering.
                            _ => panic!("Invalid shape"),
                        }
                    }
                }
                shapes.push(new_shape);
                shape = flip_present(&shape);
            }
            shape = rotate_90_cw(&shape);
        }
        shapes.dedup();

        Self { index, shapes }
    }
}

fn flip_present(present: &[String; 3]) -> [String; 3] {
    [
        present[0].chars().rev().collect(),
        present[1].chars().rev().collect(),
        present[2].chars().rev().collect(),
    ]
}

fn rotate_90_cw(shape: &[String; 3]) -> [String; 3] {
    // Create a buffer filled with empty dots (or spaces)
    let mut grid = [['.'; 3]; 3];

    for y in 0..3 {
        // Convert string row to chars to access by index
        let row_chars: Vec<char> = shape[y].chars().collect();

        for x in 0..3 {
            let char_at_pos = row_chars[x];

            let new_y = x;
            let new_x = (3 - 1) - y;

            grid[new_y][new_x] = char_at_pos;
        }
    }
    // Then make the actual output
    [
        String::from_iter(grid[0].iter()),
        String::from_iter(grid[1].iter()),
        String::from_iter(grid[2].iter()),
    ]
}

#[derive(Debug)]
struct Region {
    height: usize,
    width: usize,
    requirements: Vec<usize>,
}

#[derive(Debug)]
struct Problem {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

fn parse_input(filename: &str) -> Problem {
    let mut source = read_file(filename).into_iter();
    let mut presents = vec![];
    let mut regions = vec![];
    while let Some(line) = source.next() {
        if line.is_empty() {
            // skip the empty lines
            continue;
        }
        if let Some(caps) = REGEX.captures(&line) {
            debug_println!("{:?}", caps);
            if let Some(p_index) = &caps.name("p_index") {
                let p_index = p_index.as_str().trim_matches(':');
                let shape: [String; 3] = [
                    source.next().unwrap(),
                    source.next().unwrap(),
                    source.next().unwrap(),
                ];
                let new_present: Present = Present::new(
                    p_index
                        .parse()
                        .expect("Unable to convert present index to usize"),
                    &shape,
                );
                presents.push(new_present);
                // Problem uses 3x3 grids.
            } else if let Some(region) = &caps.name("dimensions") {
                let dimensions: Vec<usize> = region
                    .as_str()
                    .trim_matches(':')
                    .split('x')
                    .map(|x| {
                        x.parse::<usize>()
                            .expect("Unable to convert a dimension to usize")
                    })
                    .collect();
                debug_println!("{:?}", dimensions);
                let requirements: Vec<usize> = caps
                    .name("requirements")
                    .expect("Somehow got dimensions without requirements?")
                    .as_str()
                    .trim_start_matches(' ')
                    .split_whitespace()
                    .map(|f| {
                        f.parse::<usize>()
                            .expect("Unable to convert a present count to usize")
                    })
                    .collect();
                debug_println!("Requirements: {requirements:?}");
                regions.push(Region {
                    height: dimensions[0],
                    width: dimensions[1],
                    requirements,
                });
            }
        } else {
            panic!("Couldn't parse line:\n{line}")
        };
    }

    Problem { presents, regions }
}

fn main() {
    let _t = Timer::start("Day 12");
    let data = parse_input("./data/day12_test");
    debug_println!("{data:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_present() {
        let shape = [
            String::from("##."),
            String::from("#.."),
            String::from("##."),
        ];

        let get = Present::new(0, &shape);
        assert_eq!(
            get,
            Present {
                index: 0,
                shapes: vec![
                    [6, 4, 6],
                    [3, 1, 3],
                    [7, 5, 0],
                    [3, 1, 3],
                    [6, 4, 6],
                    [0, 5, 7]
                ]
            }
        );
    }

    // #[rstest]
    // fn test_part_one_from_sample_data() {
    //     let source = parse_input("./data/day12_test");
    //     assert_eq!(part_one(&source), 5);
    // }

    // #[rstest]
    // fn test_part_two_from_sample_data() {
    //     let source = parse_input("./data/day12_test");
    //     assert_eq!(part_two(&source), 2);
    // }
}
