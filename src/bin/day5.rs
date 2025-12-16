use advent_of_code_2025::*;

fn parse_file(filename: &str) -> (Vec<std::ops::RangeInclusive<usize>>, Vec<usize>) {
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

fn part_one(fresh_ranges: Vec<std::ops::RangeInclusive<usize>>, ingredients: Vec<usize>) -> usize {
    // Bit brute force.  This could get nasty on large inputs. I ought to consider sorting both lists and stopping when we're out of range.
    let _t = Timer::start("Part One");
    let mut fresh_count = 0;
    for ingredient in ingredients {
        for range in &fresh_ranges {
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
    part_one(fresh_ranges, ingredients);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one() {
        let (fresh_ranges, ingredients) = parse_file("./data/day5_test");
        assert_eq!(part_one(fresh_ranges, ingredients), 3);
    }
}
