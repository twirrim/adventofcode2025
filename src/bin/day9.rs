use advent_of_code_2025::*;

#[derive(Debug)]
struct RedTile {
    x: isize,
    y: isize,
}

impl RedTile {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn rectangle_area(&self, other: &RedTile) -> isize {
        let answer = ((other.x - self.x).abs() + 1) * ((other.y - self.y).abs() + 1);
        debug_println!(
            "Area of rectangle made from {:?} and {:?} is {answer}",
            self,
            other
        );
        answer
    }
}

fn parse_input(filename: &str) -> Vec<RedTile> {
    let _t = Timer::start(format!("Parsing file: {filename}"));
    read_file(filename)
        .iter()
        .map(|l| {
            let mut iter = l.split(',');
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            RedTile::new(x, y)
        })
        .collect()
}

fn part_one(source: &[RedTile]) -> isize {
    let _t = Timer::start("Part One");
    let sizes: Vec<isize> = source
        .iter()
        .flat_map(|f| {
            source
                .iter()
                .map(|g| f.rectangle_area(g))
                .collect::<Vec<_>>()
        })
        .collect();
    debug_println!("{:?}", sizes);
    let answer = sizes.iter().max().unwrap();
    println!("Part One Result: {answer}");
    *answer
}

fn main() {
    let _t = Timer::start("Day 9");
    let data = parse_input("./data/day9.txt");
    part_one(&data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part_one_from_sample_data() {
        let source = parse_input("./data/day9_test");
        assert_eq!(part_one(&source), 50);
    }

    // #[rstest]
    // fn test_part_two_from_sample_data() {
    //     let source = parse_input("./data/day9_test");
    //     assert_eq!(part_two(&source), 50);
    // }
}
