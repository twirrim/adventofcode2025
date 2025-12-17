use std::collections::VecDeque;

use advent_of_code_2025::*;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

type Calculation = (Operator, VecDeque<isize>);

fn split_line(line: &str) -> Vec<&str> {
    line.split(' ').filter(|x| !x.is_empty()).collect()
}

fn parse_input(filename: &str) -> Vec<Calculation> {
    let _t = Timer::start(format!("Parsing file: {filename}"));
    let mut output = vec![];
    let mut source = read_file(filename);
    // pop the last line
    let operators: Vec<Operator> = split_line(&source.pop().unwrap())
        .into_iter()
        .map(|x| match x {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => panic!("Unknown operator: {x}"),
        })
        .collect();
    // Use this to pre-populate the output
    for operator in operators {
        output.push((operator, VecDeque::new()));
    }
    // Now read through the rest of the lines, and put the contents into the appropriate Vecs.
    for line in source {
        let values = split_line(&line);
        for (idx, value) in values.iter().enumerate() {
            output[idx].1.push_back(value.parse().unwrap());
        }
        debug_println!("{:?}", values);
    }
    debug_println!("{:?}", output);
    output
}

fn calculate(calculation: &Calculation) -> isize {
    debug_println!("Calculating: {:?}", calculation);
    let mut calc = calculation.clone();
    let mut answer = calc.1.pop_front().unwrap(); // should never fail?  Or just return 0 maybe?
    for value in &calc.1 {
        match calc.0 {
            Operator::Add => answer += *value,
            Operator::Subtract => answer -= *value,
            Operator::Multiply => answer *= *value,
            Operator::Divide => answer /= *value,
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

fn main() {
    let _t = Timer::start("Day 6");
    let data = parse_input("./data/day6.txt");
    part_one(data);
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
    #[case((Operator::Add, VecDeque::from([1,2,3])), 6)]
    #[case((Operator::Subtract, VecDeque::from([1,2,3])), -4)]
    #[case((Operator::Multiply, VecDeque::from([1,2,3])), 6)]
    #[case((Operator::Divide, VecDeque::from([1,2,3])), 0)] // integer division!
    #[case((Operator::Divide, VecDeque::from([9,2,3])), 1)]
    fn test_calculate(#[case] calculation: Calculation, #[case] want: isize) {
        assert_eq!(calculate(&calculation), want)
    }

    #[rstest]
    fn test_part_one_with_example_data() {
        let calculations = parse_input("./data/day6_test");
        assert_eq!(part_one(calculations), 4277556)
    }
}
