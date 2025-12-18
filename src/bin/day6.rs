use std::collections::VecDeque;

use advent_of_code_2025::*;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
struct Calculation {
    operator: Operator,
    values: VecDeque<isize>,
}

#[inline]
fn split_line(line: &str) -> Vec<&str> {
    line.split(' ').filter(|x| !x.is_empty()).collect()
}

fn parse_operators(line: &str) -> Vec<Operator> {
    split_line(line)
        .into_iter()
        .map(|x| match x {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => panic!("Unknown operator: {x}"),
        })
        .collect()
}

fn parse_input_part_two(filename: &str) -> Vec<Calculation> {
    let _t = Timer::start(format!("Parsing file for part two: {filename}"));
    let mut output = vec![];
    let mut source = read_file(filename);
    // Collect the operators (reverse it!)
    let operators: Vec<Operator> = parse_operators(&source.pop().unwrap())
        .into_iter()
        .rev()
        .collect();
    // Use this to pre-populate the output
    for operator in operators {
        output.push(Calculation {
            operator,
            values: VecDeque::new(),
        });
    }
    // Now we need to treat the rest of the lines as if they're a 2D grid.
    // Just in case of copy/paste error, make sure we know the longest line
    let max_len = source
        .iter()
        .map(std::string::String::len)
        .max()
        .unwrap_or(0);

    // Make the grid, padding the lines as necessary
    let grid: Vec<Vec<char>> = source
        .iter()
        .map(|f| format!("{f:max_len$}").chars().collect())
        .collect();

    debug_println!("{:?}", grid);
    let row_count = grid.len();
    // Working right to left, we can figure out the dividers by looking for columns that are only space.
    let mut dividers: Vec<usize> = vec![];
    for column in (0..max_len).rev() {
        let mut divider = true;
        for row in grid.iter().take(row_count) {
            if row[column] != ' ' {
                divider = false;
                break;
            }
        }
        if divider {
            dividers.push(column);
        }
    }
    // Add the very start
    dividers.push(0);
    debug_println!("dividers: {:?}", &dividers);
    // Dividers are highest to low.  Start at the far right
    let mut start = max_len;
    let mut collected_numbers = vec![];
    for divider in dividers {
        debug_println!("Start: {start}");
        let mut number_set = VecDeque::new();
        for column in (divider..start).rev() {
            let mut number = String::new();
            for row in &grid {
                number = format!("{number}{}", row[column]);
            }
            // Strip any whitespace
            number.retain(|x| !x.is_whitespace());
            if !number.is_empty() {
                // strip the whitespace
                debug_println!("Adding {number}");
                number_set.push_back(number.parse::<isize>().unwrap());
            }
        }
        collected_numbers.push(number_set);
        start = divider;
    }
    debug_println!("Collected numbers: {:?}", collected_numbers);
    for (index, numbers) in collected_numbers.into_iter().enumerate() {
        output[index].values = numbers;
    }
    output
}

fn parse_input_part_one(filename: &str) -> Vec<Calculation> {
    let _t = Timer::start(format!("Parsing file for part one: {filename}"));
    let mut output = vec![];
    let mut source = read_file(filename);
    // pop the last line
    let operators: Vec<Operator> = parse_operators(&source.pop().unwrap());
    // Use this to pre-populate the output
    for operator in operators {
        output.push(Calculation {
            operator,
            values: VecDeque::new(),
        });
    }
    // Now read through the rest of the lines, and put the contents into the appropriate Vecs.
    for line in source {
        let values = split_line(&line);
        for (idx, value) in values.iter().enumerate() {
            output[idx].values.push_back(value.parse().unwrap());
        }
        debug_println!("{:?}", values);
    }
    debug_println!("{:?}", output);
    output
}

fn calculate(calculation: &Calculation) -> isize {
    debug_println!("Calculating: {:?}", calculation);
    let mut calc = calculation.clone();
    let mut answer = calc.values.pop_front().unwrap(); // should never fail?  Or just return 0 maybe?
    for value in calc.values {
        match calc.operator {
            Operator::Add => answer += value,
            Operator::Subtract => answer -= value,
            Operator::Multiply => answer *= value,
            Operator::Divide => answer /= value,
        }
    }
    debug_println!("Answer: {answer}");
    answer
}

fn part_one(calculations: Vec<Calculation>) -> isize {
    let _t = Timer::start("Part One");
    let mut sum = 0;
    for calculation in calculations {
        sum += calculate(&calculation);
        debug_println!("Current sum {sum}");
    }
    println!("Part One Result: {sum}");
    sum
}

fn part_two(calculations: Vec<Calculation>) -> isize {
    let _t = Timer::start("Part Two");
    let mut sum = 0;
    for calculation in calculations {
        sum += calculate(&calculation);
        debug_println!("Current sum {sum}");
    }
    println!("Part Two Result: {sum}");
    sum
}

fn main() {
    let _t = Timer::start("Day 6");
    let calculations = parse_input_part_one("./data/day6.txt");
    part_one(calculations);
    let calculations = parse_input_part_two("./data/day6.txt");
    part_two(calculations);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_split_line() {
        let line = "  6 98  215 314";
        assert_eq!(split_line(&line), vec!["6", "98", "215", "314"]);
    }

    #[rstest]
    #[case(Calculation{ operator: Operator::Add, values: VecDeque::from([1,2,3])}, 6)]
    #[case(Calculation{ operator: Operator::Subtract, values: VecDeque::from([1,2,3])}, -4)]
    #[case(Calculation{ operator: Operator::Multiply, values: VecDeque::from([1,2,3])}, 6)]
    #[case(Calculation{ operator: Operator::Divide, values: VecDeque::from([1,2,3])}, 0)]
    #[case(Calculation{ operator: Operator::Divide, values: VecDeque::from([9,2,3])}, 1)]
    fn test_calculate(#[case] calculation: Calculation, #[case] want: isize) {
        assert_eq!(calculate(&calculation), want)
    }

    #[rstest]
    fn test_part_one_with_example_data() {
        let calculations = parse_input_part_one("./data/day6_test");
        assert_eq!(part_one(calculations), 4277556)
    }

    #[rstest]
    fn test_part_two_with_example_data() {
        let calculations = parse_input_part_two("./data/day6_test");
        assert_eq!(part_two(calculations), 3263827)
    }
}
