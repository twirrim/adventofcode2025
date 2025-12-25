use itertools::Itertools;

use advent_of_code_2025::*;

#[derive(Debug, PartialEq)]
struct Machine {
    indicator_lights: Vec<bool>,
    wiring_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl Machine {
    fn new(
        indicator_lights: Vec<bool>,
        wiring_schematics: Vec<Vec<usize>>,
        joltage_requirements: Vec<usize>,
    ) -> Self {
        Self {
            indicator_lights,
            wiring_schematics,
            joltage_requirements,
        }
    }
}

fn parse_input(filename: &str) -> Vec<Machine> {
    let _t = Timer::start(format!("Parsing file {filename}"));
    let machines: Vec<Machine> = read_file(filename)
        .iter()
        .map(|f| {
            // do stuff
            let mut line = f.split(' ');
            // First line is indicators.
            let indicator_lights: Vec<bool> = line
                .next()
                .unwrap()
                .chars()
                .filter_map(|c| match c {
                    '[' | ']' => None,
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => panic!("What is: {c}?"),
                })
                .collect();
            let mut wiring_schematics = vec![];
            let mut joltage_requirements = vec![];
            for component in line {
                debug_println!("{}", component);
                if component.starts_with('(') {
                    wiring_schematics.push(
                        component
                            .trim_matches(['(', ')'])
                            .split(',')
                            .filter_map(|s| s.trim().parse().ok()) // I'm going to trust the input
                            .collect(),
                    );
                } else {
                    // assuming joltage, should be safe given what I see in the input.
                    // There's only one of these entries
                    joltage_requirements = component
                        .trim_matches(['{', '}'])
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok()) // I'm going to trust the input
                        .collect();
                }
            }
            Machine::new(indicator_lights, wiring_schematics, joltage_requirements)
        })
        .collect();
    machines
}

fn part_two(source: &[Machine]) -> usize {
    let _t = Timer::start("Part Two");
    // could parallelise this with a little bit of effort.
    // For part two, I'm pretty sure I'd want a bound / prune algorithm at the very least, to minimise time spent evaluating known bad combinations.
    // e.g. if I know (1,2), (1,2) is bad, I don't want to waste time on (1,2), (1,2), (2,3) etc.
    let answers: Vec<usize> = source
        .iter()
        .filter_map(|f| {
            println!("{:?}", f);
            for n in 1..*f.joltage_requirements.iter().max().unwrap() {
                for sequence in f.wiring_schematics.iter().combinations_with_replacement(n) {
                    let mut joltage_state = vec![0; f.joltage_requirements.len()];
                    for button in sequence {
                        for toggle in button {
                            joltage_state[*toggle] += 1;
                        }
                        // check if we should early abort
                        for (i, item) in joltage_state.iter().enumerate() {
                            if *item > f.joltage_requirements[i] {
                                break;
                            }
                        }
                    }
                    // This will never match in the "give up" state, so we're safe
                    if joltage_state == f.joltage_requirements {
                        return Some(n);
                    }
                }
            }
            panic!("Uh oh, no answers for {:?}", f);
        })
        .collect();
    let final_answer = answers.iter().sum();
    println!("Part Two Result: {final_answer}");
    final_answer
}

fn part_one(source: &[Machine]) -> usize {
    let _t = Timer::start("Part One");
    // could parallelise this with a little bit of effort.
    // Assumption: No machine will take more than 10 button presses (gut feeling is that'd be an impractical number of combinations to check)
    let answers: Vec<usize> = source
        .iter()
        .map(|f| {
            for n in 1..11 {
                for sequence in f.wiring_schematics.iter().combinations_with_replacement(n) {
                    let mut state = vec![false; f.indicator_lights.len()];
                    for button in sequence {
                        for toggle in button {
                            state[*toggle] = !state[*toggle];
                        }
                    }
                    if state == f.indicator_lights {
                        return n;
                    }
                }
            }
            panic!("Uh oh, no answers for {:?}", f);
        })
        .collect();
    let final_answer = answers.iter().sum();
    println!("Part One Result: {final_answer}");
    final_answer
}

fn main() {
    let _t = Timer::start("Day 10");
    let data = parse_input("./data/day10.txt");
    part_one(&data);
    part_two(&data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_read_file() {
        let data = parse_input("./data/day10_test");
        assert_eq!(
            data,
            [
                Machine::new(
                    vec![false, true, true, false],
                    vec![
                        vec![3],
                        vec![1, 3],
                        vec![2],
                        vec![2, 3],
                        vec![0, 2],
                        vec![0, 1]
                    ],
                    vec![3, 5, 4, 7]
                ),
                Machine::new(
                    vec![false, false, false, true, false],
                    vec![
                        vec![0, 2, 3, 4],
                        vec![2, 3],
                        vec![0, 4],
                        vec![0, 1, 2],
                        vec![1, 2, 3, 4],
                    ],
                    vec![7, 5, 12, 7, 2]
                ),
                Machine::new(
                    vec![false, true, true, true, false, true],
                    vec![
                        vec![0, 1, 2, 3, 4],
                        vec![0, 3, 4],
                        vec![0, 1, 2, 4, 5],
                        vec![1, 2],
                    ],
                    vec![10, 11, 11, 5, 10, 5]
                )
            ]
        )
    }

    #[rstest]
    fn test_part_one_from_sample_data() {
        let source = parse_input("./data/day10_test");
        assert_eq!(part_one(&source), 7);
    }

    #[rstest]
    fn test_part_two_from_sample_data() {
        let source = parse_input("./data/day10_test");
        assert_eq!(part_two(&source), 33);
    }
}
