use advent_of_code_2025::*;
use std::collections::HashMap;

// trying to avoid hashing strings
struct World {
    graph: Vec<Vec<usize>>,
    name_to_id: HashMap<String, usize>,
}

impl World {
    fn new(source: HashMap<String, Vec<String>>) -> Self {
        let mut name_to_id = HashMap::new();
        let mut graph = Vec::new();

        let get_id = |name: &String,
                      name_to_id: &mut HashMap<String, usize>,
                      graph: &mut Vec<Vec<usize>>| {
            let next_id = name_to_id.len();
            let id = *name_to_id.entry(name.clone()).or_insert(next_id);

            // If we just added a new ID, expand the graph to accommodate it
            if id >= graph.len() {
                graph.resize(id + 1, vec![]);
            }
            id
        };

        for (source_node, neighbors) in source {
            let u = get_id(&source_node, &mut name_to_id, &mut graph);

            for neighbor in neighbors {
                let v = get_id(&neighbor, &mut name_to_id, &mut graph);
                graph[u].push(v);
            }
        }

        Self { graph, name_to_id }
    }
}

fn parse_input(filename: &str) -> World {
    let _t = Timer::start(format!("Parsing file {filename}"));
    World::new(
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
            .collect(),
    )
}

fn find_all_paths(
    world: &World,
    start_node: &str,
    end_node: &str,
    must_contain: Vec<&str>,
) -> usize {
    let start = world.name_to_id[start_node];
    let end = world.name_to_id[end_node];
    let must_contain = must_contain
        .iter()
        .map(|f| world.name_to_id[*f])
        .collect::<Vec<_>>();

    let mut path_count = 0;
    let mut stack = vec![(start, vec![start])];

    while let Some((node, path)) = stack.pop() {
        // This will never fail with our data
        for neighbour in &world.graph[node] {
            let mut found_path = path.clone();
            found_path.push(neighbour.to_owned());
            if neighbour == &end {
                let mut good = true;
                for requirement in &must_contain {
                    if !found_path.contains(&requirement) {
                        good = false;
                        break;
                    }
                }
                if good {
                    path_count += 1;
                    debug_println!("Found {path_count} paths so far");
                }
            } else {
                stack.push((*neighbour, found_path));
            }
        }
    }

    path_count
}

fn part_two(world: &World) -> usize {
    let _t = Timer::start("Part Two");
    let answer = find_all_paths(world, "svr", "out", vec!["dac", "fft"]);
    println!("Part Two Result: {answer}");
    answer
}

fn part_one(world: &World) -> usize {
    let _t = Timer::start("Part One");
    let answer = find_all_paths(world, "you", "out", vec![]);
    println!("Part One Result: {answer}");
    answer
}

fn main() {
    let _t = Timer::start("Day 11");
    let world = parse_input("./data/day11.txt");
    part_one(&world);
    part_two(&world);
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

    #[rstest]
    fn test_part_two_from_sample_data() {
        let source = parse_input("./data/day11_2_test");
        assert_eq!(part_two(&source), 2);
    }
}
