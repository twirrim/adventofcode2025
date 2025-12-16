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

fn part_one(data: &[Vec<char>]) -> usize {
    let _t = Timer::start("Part One");
    debug_println!("Evaluating {data:?}");
    // iterate over each entry
    let mut can_be_moved = 0;
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            debug_println!("{x},{y}");
            if data[x][y] == '@' {
                // find indexes of the adjacent locations
                let adjacent = get_adjacent_indexes(data, x, y);
                let mut count = 0;
                for (x_idx, y_idx) in adjacent {
                    if data[x_idx][y_idx] == '@' {
                        count += 1;
                    }
                }
                if count < 4 {
                    debug_println!("{x},{y} is safe to move");
                    can_be_moved += 1;
                }
            }
        }
    }
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
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one_test_input() {
        assert_eq!(part_one(&parse_input("./data/day4_test")), 13);
    }
}
