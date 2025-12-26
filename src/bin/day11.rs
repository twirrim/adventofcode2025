use advent_of_code_2025::*;
use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;

fn parse_input(filename: &str) -> Graph {
    let _t = Timer::start(format!("Parsing file {filename}"));
    read_file(filename)
        .iter()
        .map(|line| {
            let split_whitespace = line.split_whitespace().collect::<Vec<_>>();
            let mut line_iter = split_whitespace.iter();
            let key = line_iter.next().unwrap().trim_matches(':');
            (
                key.to_string(),
                line_iter.map(std::string::ToString::to_string).collect(),
            )
        })
        .collect()
}

fn find_all_paths(graph: &Graph, start: &String, end: &String) -> Vec<Vec<String>> {
    let mut stack = vec![(start, vec![start.to_owned()])];
    let mut results = vec![];
    while let Some((node, path)) = stack.pop() {
        // This will never fail with our data
        for neighbour in graph.get(node).unwrap() {
            let mut found_path = path.clone();
            found_path.push(neighbour.to_owned());
            if neighbour == end {
                results.push(found_path);
            } else {
                stack.push((neighbour, found_path));
            }
        }
    }

    results
}

fn part_one(data: &Graph) -> usize {
    let _t = Timer::start("Part One");
    let paths = find_all_paths(data, &"you".to_string(), &"out".to_string());
    let answer = paths.len();
    println!("Part One Result: {answer}");
    answer
}

fn main() {
    let _t = Timer::start("Day 11");
    let data = parse_input("./data/day11.txt");
    part_one(&data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one_from_sample_data() {
        let source = parse_input("./data/day11_test");
        assert_eq!(part_one(&source), 5);
    }

    // #[rstest]
    // fn test_part_two_from_sample_data() {
    //     let source = parse_input("./data/day9_test");
    //     assert_eq!(part_two(&source), 24);
    // }
}
