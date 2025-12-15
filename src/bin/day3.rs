use std::char;

use indicatif::{ProgressIterator, ProgressStyle};

use advent_of_code_2025::*;

fn parse_input(filename: &str) -> Vec<String> {
    let _t = Timer::start(format!("Parsing file: {}", filename));
    read_file(filename)
}

fn part_two_evaluate_brute_force(bank: &str) -> usize {
    let mut max = usize::MIN;
    let letters: Vec<char> = bank.chars().collect();
    let styles: Vec<ProgressStyle> = (0..12)
        .map(|f| {
            let base_string =
                "[{elapsed_precise}/{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}";
            let template_string = format!("{base_string} {f}");
            ProgressStyle::with_template(&template_string).unwrap()
        })
        .collect();
    for (first_idx, first) in letters
        .iter()
        .progress_with_style(styles[0].clone())
        .enumerate()
    {
        for (second_idx, second) in letters
            .iter()
            .skip(first_idx + 1)
            .progress_with_style(styles[1].clone())
            .enumerate()
        {
            for (third_idx, third) in letters
                .iter()
                .skip(second_idx + 1)
                .progress_with_style(styles[2].clone())
                .enumerate()
            {
                for (fourth_idx, fourth) in letters
                    .iter()
                    .skip(third_idx + 1)
                    .progress_with_style(styles[3].clone())
                    .enumerate()
                {
                    for (fifth_idx, fifth) in letters
                        .iter()
                        .skip(fourth_idx + 1)
                        .progress_with_style(styles[4].clone())
                        .enumerate()
                    {
                        for (sixth_idx, sixth) in letters
                            .iter()
                            .skip(fifth_idx + 1)
                            .progress_with_style(styles[5].clone())
                            .enumerate()
                        {
                            for (seventh_idx, seventh) in letters
                                .iter()
                                .skip(sixth_idx + 1)
                                .progress_with_style(styles[6].clone())
                                .enumerate()
                            {
                                for (eigth_idx, eigth) in letters
                                    .iter()
                                    .skip(seventh_idx + 1)
                                    .progress_with_style(styles[7].clone())
                                    .enumerate()
                                {
                                    for (ninth_idx, ninth) in
                                        letters.iter().skip(eigth_idx + 1).enumerate()
                                    {
                                        for (tenth_idx, tenth) in
                                            letters.iter().skip(ninth_idx + 1).enumerate()
                                        {
                                            for (eleventh_idx, eleventh) in
                                                letters.iter().skip(tenth_idx + 1).enumerate()
                                            {
                                                for twelth in letters.iter().skip(eleventh_idx + 1)
                                                {
                                                    let joltage: usize = format!(
                                                        "{}{}{}{}{}{}{}{}{}{}{}{}",
                                                        first,
                                                        second,
                                                        third,
                                                        fourth,
                                                        fifth,
                                                        sixth,
                                                        seventh,
                                                        eigth,
                                                        ninth,
                                                        tenth,
                                                        eleventh,
                                                        twelth
                                                    )
                                                    .parse()
                                                    .unwrap();
                                                    if joltage > max {
                                                        max = joltage;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    max
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

fn part_two(banks: &[String]) -> usize {
    let _t = Timer::start("Part Two");
    let answer: usize = banks.iter().map(|f| part_two_evaluate_brute_force(f)).sum();
    println!("Part Two: {answer}");
    answer
}

fn main() {
    let _timer = Timer::start("Day 3");
    let banks = parse_input("./data/day3.txt");
    debug_println!("{:?}", banks);
    part_one(&banks);
    part_two(&banks);
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

    #[rstest]
    fn test_part_two_using_test_data() {
        let banks = parse_input("./data/day3_test");
        assert_eq!(part_two(&banks), 3121910778619);
    }
}
