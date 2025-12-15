use std::char;

use advent_of_code_2025::*;

fn parse_input(filename: &str) -> Vec<String> {
    let _t = Timer::start(format!("Parsing file: {}", filename));
    read_file(filename)
}

fn part_one_evaluate_bank(bank: &str) -> usize {
    let mut max = usize::MIN;
    let letters: Vec<char> = bank.chars().collect();
    for (idx, first) in letters.iter().enumerate() {
        for second in letters.iter().skip(idx + 1) {
            let joltage: usize = format!("{}{}", first, second).parse().unwrap();
            debug_println!("Joltage: {joltage}");
            if joltage > max {
                debug_println!(
                    "current joltage {} > current max {}, updating max",
                    joltage,
                    max
                );
                max = joltage;
            }
        }
    }
    debug_println!("Returning max joltage: {max}");
    max
}

fn part_one(banks: &[String]) -> usize {
    let _t = Timer::start("Part One");
    let answer: usize = banks.iter().map(|f| part_one_evaluate_bank(f)).sum();
    println!("Part One: {answer}");
    answer
}

fn main() {
    let _timer = Timer::start("Day 3");
    let banks = parse_input("./data/day3.txt");
    debug_println!("{:?}", banks);
    part_one(&banks);
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_known_bank() {
        assert_eq!(part_one_evaluate_bank(&String::from("234234234234278")), 78);
    }

    #[rstest]
    fn test_part_one_using_test_data() {
        let banks = parse_input("./data/day3_test");
        assert_eq!(part_one(&banks), 357);
    }
}
