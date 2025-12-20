use std::fmt;

use advent_of_code_2025::*;

#[derive(Debug, PartialEq, Clone)]
enum Contents {
    Emitter,
    Beam,
    Splitter,
    Space,
}

impl fmt::Display for Contents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Contents::Beam => '|',
            Contents::Emitter => 'S',
            Contents::Space => '.',
            Contents::Splitter => '^',
        };
        write!(f, "{c}")
    }
}

struct Map(Vec<Vec<Contents>>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(f, "{col}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(filename: &str) -> Map {
    let _t = Timer::start(format!("Parsing file {filename}"));
    let source = read_file(filename);
    Map(source
        .iter()
        .map(|f| {
            f.chars()
                .map(|c| match c {
                    '.' => Contents::Space,
                    '^' => Contents::Splitter,
                    '|' => Contents::Beam,
                    'S' => Contents::Emitter,
                    _ => panic!("Unknown character {c}"),
                })
                .collect()
        })
        .collect())
}

fn part_one(map: &Map) -> usize {
    let _t = Timer::start("Part One");
    let mut map = map.0.clone();
    let mut count = 0;
    let row_count = map.len();

    for r_idx in 0..row_count - 1 {
        debug_println!("Current state:\n{}", Map(map.clone()));
        for c_idx in 0..map[r_idx].len() {
            match map[r_idx][c_idx] {
                Contents::Space | Contents::Splitter => (),
                Contents::Beam | Contents::Emitter => {
                    // going to make a dangerous assumption that no splitter exists on the edges of the map.
                    // Only Space and Splitter could exist in the next row
                    match map[r_idx + 1][c_idx] {
                        Contents::Space => map[r_idx + 1][c_idx] = Contents::Beam,
                        Contents::Splitter => {
                            count += 1;
                            // TODO: What if either is a splitter? I think this may need to be recursive!
                            assert!(
                                map[r_idx + 1][c_idx - 1] != Contents::Splitter,
                                "Splitter at r_idx+1, c_idx-1, where we were going to put a beam"
                            );
                            assert!(
                                map[r_idx + 1][c_idx + 1] != Contents::Splitter,
                                "Splitter at r_idx+1, c_idx+1, where we were going to put a beam"
                            );
                            map[r_idx + 1][c_idx - 1] = Contents::Beam;
                            map[r_idx + 1][c_idx + 1] = Contents::Beam;
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    debug_println!("Final state:\n{}", Map(map.clone()));
    println!("Part One Result: {count}");
    count
}

fn part_two(map: &Map) -> usize {
    let _t = Timer::start("Part Two");
    let map = map.0.clone();
    let row_count = map.len();
    let mut count_map: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map[0].len()];
    // Set the emitter to 1.  There will always be an emitter in the first line
    count_map[0][map[0].iter().position(|f| *f == Contents::Emitter).unwrap()] = 1;
    for r_idx in 1..row_count - 1 {
        //debug_println!("{:?}", print_interleved_count_map(&count_map, &map));
        for c_idx in 0..map[r_idx].len() {
            //debug_println!("{r_idx},{c_idx}");
            match map[r_idx][c_idx] {
                Contents::Splitter => {
                    // I've validated in the data source that no splitter appears at the edges
                    let prev_count = count_map[r_idx - 1][c_idx];
                    count_map[r_idx][c_idx - 1] += prev_count;
                    count_map[r_idx][c_idx + 1] += prev_count;
                }
                _ => {
                    count_map[r_idx][c_idx] += count_map[r_idx - 1][c_idx];
                }
            }
        }
    }
    let answer: usize = count_map.last().unwrap().iter().sum::<usize>();
    println!("Part Two Result: {answer}");
    answer
}

fn main() {
    let _t = Timer::start("Day 7");
    let source: Map = parse_input("./data/day7.txt");
    debug_println!("{source}");
    part_one(&source);
    part_two(&source);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one_from_sample_data() {
        let source: Map = parse_input("./data/day7_test");
        assert_eq!(part_one(&source), 21);
    }

    #[rstest]
    fn test_part_two_from_sample_data() {
        let source: Map = parse_input("./data/day7_test");
        assert_eq!(part_two(&source), 40);
    }
}
