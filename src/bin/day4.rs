use advent_of_code_2025::*;

// I've a niggling feeling this'll come up again.  I think it has in previous years
// If It does, I'll move this to the library
fn get_adjacent_indexes(data: &[Vec<char>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let rows = data.len();
    // base case
    if rows == 0 {
        return vec![];
    }

    let cols = data[0].len();

    let mut adjacent = Vec::new();
    for row_delta in -1..=1 {
        for col_delta in -1..=1 {
            // skip self
            if row_delta == 0 && col_delta == 0 {
                continue;
            }
            let new_x = x as isize + row_delta;
            let new_y = y as isize + col_delta;

            // Check if they're out of bounds
            if new_x >= 0 && new_x < rows as isize && new_y >= 0 && new_y < cols as isize {
                // this should be safe for our purposes. Could overflow if the number of rows is crazy size
                adjacent.push((new_x as usize, new_y as usize));
            }
        }
    }
    adjacent
}

fn evaluate_map(map: &Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    let mut new_map = map.clone();
    let mut can_be_moved = 0;
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            debug_println!("{x},{y}");
            if map[x][y] == '@' {
                // find indexes of the adjacent locations
                let adjacent = get_adjacent_indexes(map, x, y);
                let mut count = 0;
                for (x_idx, y_idx) in adjacent {
                    if map[x_idx][y_idx] == '@' {
                        count += 1;
                    }
                }
                if count < 4 {
                    debug_println!(
                        "{x},{y} is safe to move. Updating map and incrementing counter"
                    );
                    new_map[x][y] = '.';
                    can_be_moved += 1;
                }
            }
        }
    }
    (can_be_moved, new_map)
}

fn part_two(data: &Vec<Vec<char>>) -> usize {
    let _t = Timer::start("Part Two");
    let mut final_count = 0;
    let mut map = data.clone();
    loop {
        let (can_be_moved, new_map) = evaluate_map(&map);
        map = new_map;
        if can_be_moved == 0 {
            break;
        } else {
            final_count += can_be_moved;
        }
    }
    println!("Part Two result: {final_count}");
    final_count
}

fn part_one(data: &Vec<Vec<char>>) -> usize {
    let _t = Timer::start("Part One");
    let (can_be_moved, _new_map) = evaluate_map(data);
    println!("Part One result: {can_be_moved}");
    can_be_moved
}

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let _t = Timer::start(format!("Parsing {filename}"));
    read_file(filename)
        .into_iter()
        .map(|x| x.chars().collect())
        .collect()
}

fn main() {
    let _t = Timer::start("Day 4");
    let data = parse_input("./data/day4.txt");
    part_one(&data);
    part_two(&data);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one_test_input() {
        assert_eq!(part_one(&parse_input("./data/day4_test")), 13);
    }

    #[rstest]
    fn test_part_two_test_input() {
        assert_eq!(part_two(&parse_input("./data/day4_test")), 43);
    }
}
