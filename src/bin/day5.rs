use std::ops::RangeInclusive;

use advent_of_code_2025::*;

fn parse_file(filename: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let _t = Timer::start(format!("Parsing file: {filename}"));
    let mut fresh_ranges = vec![];
    let mut ingredients = vec![];
    let source = read_file(filename);
    for line in source {
        if line.is_empty() {
            continue;
        } else if line.contains('-') {
            // if it has a -, it's a range
            let details = line
                .split("-") // each entry takes the form "start_number dash end_number", so we need to parse those digits.
                .map(|digit| digit.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            debug_println!("Creating inclusive range from {:?}", details);
            fresh_ranges.push(details[0]..=details[1]);
        } else {
            debug_println!("Got ingredient id: {line}");
            ingredients.push(line.parse().unwrap());
        }
    }
    (fresh_ranges, ingredients)
}

#[inline]
fn pretty_string_range_inclusive<T: std::fmt::Display>(range: &RangeInclusive<T>) -> String {
    format!(
        "{} - {}",
        print_with_thousands_separator(range.start()),
        print_with_thousands_separator(range.end())
    )
}

fn part_two(fresh_ranges: &[RangeInclusive<usize>]) -> usize {
    let _t = Timer::start("Part Two");
    let mut sorted_ranges = fresh_ranges.to_owned();
    sorted_ranges.sort_by_key(|f| *f.start());

    let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();

    // Handle empty input case (should never happen...)
    if sorted_ranges.is_empty() {
        return 0;
    }

    // Now we need to pick a range as the "current" one, and then loop over each other range in turn.
    // If the comparison range is contained within the current range, move on
    // If the comparison range starts within the current one, and ends after it, extend the current range to match the end of the comparison
    // If the comparison range starts *after* the current one, put the current one into merged_ranges, and make the comparison one the current one.
    // Continue until we've evaluated every range.
    let mut current_start = *sorted_ranges[0].start();
    let mut current_end = *sorted_ranges[0].end();

    for range in sorted_ranges {
        debug_println!(
            "Evaluating {} against {} - {}",
            pretty_string_range_inclusive(&range),
            print_with_thousands_separator(current_start),
            print_with_thousands_separator(current_end)
        );
        if *range.start() >= current_start && *range.end() <= current_end {
            // It's a subset, skip
            debug_println!("Subset!");
            continue;
        } else if *range.start() >= current_start && *range.start() <= current_end {
            // end must be beyond current end
            debug_println!(
                "Extending current_end to {}",
                print_with_thousands_separator(*range.end())
            );
            current_end = *range.end();
        } else if *range.start() > current_end {
            // end of the line with this current range
            debug_println!("We're finished with the current range");
            merged_ranges.push(current_start..=current_end);
            current_start = *range.start();
            current_end = *range.end();
        }
    }

    // Push the final range
    merged_ranges.push(current_start..=current_end);

    let mut count = 0;

    for range in merged_ranges {
        // Annoyingly, no "len" in rust for RangeInclusive
        count += (range.end() - range.start()) + 1;
    }
    println!("Part Two result: {count}");
    count
}

fn part_one(fresh_ranges: &Vec<RangeInclusive<usize>>, ingredients: Vec<usize>) -> usize {
    // Bit brute force.  This could get nasty on large inputs.
    // worst case scenario is ingredients * fresh_ranges.
    // I ought to consider sorting both lists and stopping when we're out of range.
    let _t = Timer::start("Part One");
    let mut fresh_count = 0;
    for ingredient in ingredients {
        for range in fresh_ranges {
            if range.contains(&ingredient) {
                debug_println!("{range:?} contains {ingredient:?}");
                fresh_count += 1;
                break; // don't need to re-evaluate the same ingredient
            }
        }
    }
    println!("Part One result: {fresh_count}");
    fresh_count
}

fn main() {
    let _t = Timer::start("Day 5");
    let (fresh_ranges, ingredients) = parse_file("./data/day5.txt");
    part_one(&fresh_ranges, ingredients);
    part_two(&fresh_ranges);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one() {
        let (fresh_ranges, ingredients) = parse_file("./data/day5_test");
        assert_eq!(part_one(&fresh_ranges, ingredients), 3);
    }

    #[rstest]
    fn test_part_two() {
        let (fresh_ranges, _ingredients) = parse_file("./data/day5_test");
        assert_eq!(part_two(&fresh_ranges), 14);
    }
}
