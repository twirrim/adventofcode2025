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
    values: Vec<isize>,
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

fn parse_input_part_two(source: &[String]) -> Vec<Calculation> {
    let _t = Timer::start("Parsing file for part two");
    let mut source = source.to_owned();
    // Extract the operators in the last line, reversed for later logic
    let operators: Vec<Operator> = parse_operators(&source.pop().expect("File Empty?"))
        .into_iter()
        .rev()
        .collect();

    // Create a flat byte grid for speed
    let row_count = source.len();
    let max_len = source
        .iter()
        .map(std::string::String::len)
        .max()
        .unwrap_or(0);
    let mut grid = vec![b' '; row_count * max_len];

    for (r, line) in source.iter().enumerate() {
        let bytes = line.as_bytes();
        let start = r * max_len;
        // Copy the line bytes into our flat grid
        grid[start..start + bytes.len()].copy_from_slice(bytes);
    }

    // Find dividers (columns that are only spaces)
    let mut dividers: Vec<usize> = vec![0];
    for col in 0..max_len {
        let mut is_divider = true;
        for row in 0..row_count {
            if grid[row * max_len + col] != b' ' {
                is_divider = false;
                break;
            }
        }
        if is_divider {
            dividers.push(col);
        }
    }
    dividers.push(max_len);

    // Extract numbers
    let mut num_buf = String::with_capacity(row_count);
    let mut all_number_sets = Vec::with_capacity(operators.len());

    // Use windows to get the ranges between dividers
    dividers.sort_unstable();
    for range in dividers.windows(2).rev() {
        let left = range[0];
        let right = range[1];
        let mut number_set = Vec::new();

        for col in (left..right).rev() {
            num_buf.clear();
            for row in 0..row_count {
                let byte = grid[row * max_len + col];
                if byte != b' ' {
                    num_buf.push(byte as char);
                }
            }

            if !num_buf.is_empty()
                && let Ok(num) = num_buf.parse::<isize>()
            {
                number_set.push(num);
            }
        }

        if !number_set.is_empty() {
            all_number_sets.push(number_set);
        }
    }

    operators
        .into_iter()
        .enumerate()
        .map(|(i, operator)| {
            // Use remove if exists, otherwise empty
            let values = if i < all_number_sets.len() {
                std::mem::take(&mut all_number_sets[i])
            } else {
                Vec::new()
            };
            Calculation { operator, values }
        })
        .collect()
}

fn parse_input_part_one(source: &[String]) -> Vec<Calculation> {
    let _t = Timer::start("Parsing source for part one");
    let mut source = source.to_owned();
    let operators = parse_operators(&source.pop().expect("File empty"));

    let mut values_grid: Vec<Vec<isize>> = vec![vec![]; operators.len()];

    for line in source {
        for (idx, val) in split_line(&line).into_iter().enumerate() {
            if idx < values_grid.len() {
                values_grid[idx].push(val.parse().unwrap());
            }
        }
    }

    operators
        .into_iter()
        .zip(values_grid)
        .map(|(operator, values)| Calculation { operator, values })
        .collect()
}

fn calculate(calc: &Calculation) -> isize {
    let mut iter = calc.values.iter();
    let first = *iter.next().unwrap_or(&0);

    iter.fold(first, |accumulated, &next| match calc.operator {
        Operator::Add => accumulated + next,
        Operator::Subtract => accumulated - next,
        Operator::Multiply => accumulated * next,
        Operator::Divide => accumulated / next,
    })
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
    let file_timer = Timer::start("Reading file");
    let source = read_file("./data/day6.txt");
    std::mem::drop(file_timer);
    let calculations = parse_input_part_one(&source);
    part_one(calculations);
    let calculations = parse_input_part_two(&source);
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
    #[case(Calculation{ operator: Operator::Add, values: Vec::from([1,2,3])}, 6)]
    #[case(Calculation{ operator: Operator::Subtract, values: Vec::from([1,2,3])}, -4)]
    #[case(Calculation{ operator: Operator::Multiply, values: Vec::from([1,2,3])}, 6)]
    #[case(Calculation{ operator: Operator::Divide, values: Vec::from([1,2,3])}, 0)]
    #[case(Calculation{ operator: Operator::Divide, values: Vec::from([9,2,3])}, 1)]
    fn test_calculate(#[case] calculation: Calculation, #[case] want: isize) {
        assert_eq!(calculate(&calculation), want)
    }

    #[rstest]
    fn test_part_one_with_example_data() {
        let source = read_file("./data/day6_test");
        let calculations = parse_input_part_one(&source);
        assert_eq!(part_one(calculations), 4277556)
    }

    #[rstest]
    fn test_part_two_with_example_data() {
        let source = read_file("./data/day6_test");
        let calculations = parse_input_part_two(&source);
        assert_eq!(part_two(calculations), 3263827);
    }
}
