use std::ops::RangeInclusive;

use advent_of_code_2025::*;

fn parse_input(filename: &str) -> Vec<RangeInclusive<usize>> {
    let _timer = Timer::start("Parsing input".to_owned());
    debug_println!("Reading {}", filename);
    let source = read_file(filename);
    let line = source.first().unwrap(); // Input only has a single line
    debug_println!("Converting line into a vec of inclusive ranges");
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

fn remove_first_and_last_letters(id: &str) -> String {
    let mut id = id.to_string();
    id.pop(); // remove last
    if id.len() > 0 {
        id.remove(0); // remove first
    }
    id
}

fn invalid_id_part_two(id: usize) -> bool {
    debug_println!("Evaluating {id} using part two approach");
    // Use the part one approach first
    if invalid_id_part_one(id) {
        return true;
    }
    // Then use the part two extension
    // This is taking advantage of the "string doubling" technique I found searching for more optimal approaches than brute forcing it
    // Essentially, if a string S is made of repeating patterns, then the string will exist inside S+S that has first and last letters trimmed from it
    let doubled_id: String = format!("{id}{id}");
    debug_println!("Doubled string: {}", doubled_id);
    let stripped = remove_first_and_last_letters(&doubled_id);
    debug_println!("After stripping first and last: {stripped}");
    if stripped.contains(&format!("{id}")) {
        return true;
    }
    false
}

fn invalid_id_part_one(id: usize) -> bool {
    debug_println!("Evaluating {id} using part one approach");
    let text = format!("{id}");
    let (first, second) = text.split_at(text.len() / 2);
    debug_println!("First: {first}. Second: {second}");
    if !second.starts_with("0") {
        if first == second {
            debug_println!("Invalid ID found: {first} == {second}");
            return true;
        }
    }
    false
}

fn part_one(data: &Vec<RangeInclusive<usize>>) -> usize {
    let _timer = Timer::start("Part One".to_owned());
    let mut tally = 0;
    for range in data {
        debug_println!("Evaluating range\t{:?}", range);
        for id in range.clone().into_iter() {
            if invalid_id_part_one(id) {
                debug_println!("Invalid ID found: {id}");
                tally += id;
            }
        }
    }
    println!("Part One Result: {tally}");
    tally
}

fn part_two(data: &Vec<RangeInclusive<usize>>) -> usize {
    let _timer = Timer::start("Part Two".to_owned());
    let mut tally = 0;
    for range in data {
        debug_println!("Evaluating range\t{:?}", range);
        for id in range.clone().into_iter() {
            if invalid_id_part_two(id) {
                debug_println!("Invalid ID found: {id}");
                tally += id;
            }
        }
    }

    println!("Part Two Result: {tally}");

    tally
}

fn main() {
    let _timer = Timer::start("Day Two".to_owned());
    let data = parse_input("./data/day2.txt");
    debug_println!("{:?}", data);
    part_one(&data);
    part_two(&data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_sample_data_part_one() {
        let data = parse_input("./data/day2_test");
        assert_eq!(part_one(&data), 1227775554);
    }

    #[rstest]
    fn test_sample_data_part_two() {
        let data = parse_input("./data/day2_test");
        assert_eq!(part_two(&data), 4174379265);
    }

    #[rstest]
    #[case(11, true)]
    #[case(12, false)]
    #[case(13, false)]
    #[case(14, false)]
    #[case(21, false)]
    #[case(22, true)]
    #[case(98, false)]
    #[case(99, true)]
    #[case(100, false)]
    fn test_part_one_id_evaluation(#[case] id: usize, #[case] want: bool) {
        assert_eq!(invalid_id_part_one(id), want);
    }

    #[rstest]
    #[case(11, true)] // Still works with part one evaluation
    #[case(12, false)]
    #[case(121212, true)] // new behaviour
    #[case(121312, false)] // new behaviour
    fn test_part_two_id_evaluation(#[case] id: usize, #[case] want: bool) {
        assert_eq!(invalid_id_part_two(id), want);
    }

    #[rstest]
    fn test_remove_first_and_last_letters() {
        assert_eq!(remove_first_and_last_letters("foo"), "o");
        assert_eq!(remove_first_and_last_letters("foobar"), "ooba");
        assert_eq!(remove_first_and_last_letters("123456"), "2345");
    }
}
