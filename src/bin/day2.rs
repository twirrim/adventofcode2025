use std::ops::RangeInclusive;

use advent_of_code_2025::*;

fn parse_input(filename: &str) -> Vec<RangeInclusive<usize>> {
    debug_println!("Reading {}", filename);
    let source = read_file(filename);
    let line = source.first().unwrap(); // Input only has a single line
    line.split(",") // entries are separated by commas
        .map(|x| {
            x.split("-") // each entry takes the form "start_number dash end_number", so we need to parse those digits.
                .map(|digit| digit.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|entry| {
            // Then we take those pairings of digits and turn them into std:ops::Ranges.
            let start: usize = entry[0];
            let end: usize = entry[1];
            start..=end
        })
        .collect()
}

fn part_one(data: &Vec<RangeInclusive<usize>>) -> usize {
    let timer = Timer::start("Part One".to_owned());
    let mut tally = 0;
    for range in data {
        debug_println!("Evaluating range\t{:?}", range);
        for id in range.clone().into_iter() {
            debug_println!("Evaluating {id}");
            let text = format!("{id}");
            let (first, second) = text.split_at(text.len() / 2);
            debug_println!("First: {first}. Second: {second}");
            // Only evaluate where the second half doesn't start with a 0
            if !second.starts_with("0") {
                if first == second {
                    debug_println!("Invalid ID found: {first} == {second}");
                    tally += id;
                }
            }
        }
    }
    println!("Part One Result: {tally}");
    timer.elapsed();
    tally
}

fn part_two(data: &Vec<RangeInclusive<usize>>) -> usize {
    let timer = Timer::start("Part Two".to_owned());
    let mut tally = 0;

    println!("Part Two Result: {tally}");

    timer.elapsed();
    tally
}

fn main() {
    let timer = Timer::start("Day Two".to_owned());
    let data = parse_input("./data/day2.txt");
    debug_println!("{:?}", data);
    part_one(&data);
    timer.elapsed();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_sample_data() {
        let data = parse_input("./data/day2_test");
        assert_eq!(part_one(&data), 1227775554);
    }
}
