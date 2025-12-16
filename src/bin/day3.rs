use std::char;

use advent_of_code_2025::*;

fn parse_input(filename: &str) -> Vec<String> {
    let _t = Timer::start(format!("Parsing file: {}", filename));
    read_file(filename)
}

fn evaluate_bank(bank: &str, target_length: usize) -> usize {
    let letters_char: Vec<char> = bank.chars().collect();
    debug_println!("Bank: {letters_char:?}");
    let letters: Vec<u32> = bank.chars().map(|f| f.to_digit(10).unwrap()).collect();
    let mut chosen_indexes: Vec<usize> = vec![];
    let mut start_index: usize = 0;
    let mut answer = String::new();
    loop {
        if chosen_indexes.len() >= target_length {
            break;
        }
        // Define the range to evaluate
        if !chosen_indexes.is_empty() {
            // unwrap is safe here because previously we handled the empty case.
            start_index = chosen_indexes.last().copied().unwrap() + 1;
        }
        let end_index = letters.len() - target_length + chosen_indexes.len();
        let range = start_index..=end_index;
        debug_println!("Evaluating range {:?}", range);
        // evaluate the range
        let max_index = &letters[range]
            .iter()
            .enumerate()
            // This is annoying.  max_by_key returns last matching index, min returns first.
            // Stdlib's Reverse enables biggest to seem like smallest.
            .min_by_key(|&(_idx, &val)| std::cmp::Reverse(val)) // find the "largest" number
            .map(|(idx, _val)| idx + start_index) // drop the value and return the index + offset.
            .unwrap(); // Shouldn't be possible to panic.
        debug_println!(
            "Chosen index: {:?}, which has value {:?}",
            *max_index,
            letters_char[*max_index]
        );
        chosen_indexes.push(*max_index);
        answer += &letters_char[*max_index].to_string();
        debug_println!("Answer so far: {answer}");
    }
    // Convert the answer back to a number
    answer.parse().unwrap()
}

fn part_one(banks: &[String]) -> usize {
    let _t = Timer::start("Part One");
    let answer: usize = banks.iter().map(|f| evaluate_bank(f, 2)).sum();
    println!("Part One: {answer}");
    answer
}

fn part_two(banks: &[String]) -> usize {
    let _t = Timer::start("Part Two");
    let answer: usize = banks.iter().map(|f| evaluate_bank(f, 12)).sum();
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
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_known_bank_part_one(#[case] input: &str, #[case] want: usize) {
        assert_eq!(evaluate_bank(input, 2), want);
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_known_bank_part_two(#[case] input: &str, #[case] want: usize) {
        assert_eq!(evaluate_bank(input, 12), want);
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
