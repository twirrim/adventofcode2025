use advent_of_code_2025::{debug_println, read_file};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Lock {
    location: usize,
    zero_count: usize,
    zero_passed: usize,
}

impl Default for Lock {
    fn default() -> Self {
        Lock::new(50, 0, 0)
    }
}

impl Lock {
    fn new(location: usize, zero_count: usize, zero_passed: usize) -> Self {
        Self {
            location,
            zero_count,
            zero_passed,
        }
    }

    fn turn(&mut self, instruction: &Instruction) {
        debug_println!(
            "###############\nLock: {:?}\nInstruction: {:?}",
            self,
            instruction
        );
        // scale down the amount as appropriate
        let amount_hundreds = instruction.amount / 100;
        let amount = instruction.amount - (amount_hundreds * 100);
        if amount_hundreds != 0 {
            debug_println!(
                "There are {} hundreds in {}.  Increasing zero_passed by {}",
                amount_hundreds,
                instruction.amount,
                amount_hundreds
            );
            // Increment the 0s passed by this amount
            self.zero_passed += amount_hundreds;
            debug_println!("Zero passed now: {}", self.zero_passed);
            debug_println!("Remaining to turn: {}", amount);
        }
        let mut passed_through_zero: bool = false;
        match instruction.direction {
            Direction::Left => {
                debug_println!("Turning Left {}", amount);
                // Have it roll past 0
                if self.location < amount {
                    debug_println!(
                        "location {} < turn amount {}, so we're rolling past 0. We need to increase location by 100",
                        self.location,
                        amount
                    );
                    if self.location != 0 {
                        passed_through_zero = true
                    }
                    self.location += 100;
                }
                self.location -= amount;
                debug_println!("After moving left by {} we're at {}", amount, self.location);
            }
            Direction::Right => {
                debug_println!("Turning Right {}", amount);
                self.location += amount;
                // check if we've rolled past 99
                if self.location > 99 {
                    debug_println!(
                        "{} > 99, so we decrease by 100 to take us back to the start",
                        self.location
                    );
                    if self.location != 0 {
                        passed_through_zero = true
                    }
                    self.location -= 100;
                }
                debug_println!(
                    "After moving right by {} we're at {}",
                    amount,
                    self.location
                );
            }
        }
        if self.location == 0 {
            debug_println!("We're at 0, incrementing zero_count");
            self.zero_count += 1;
            passed_through_zero = true;
        }
        if passed_through_zero {
            debug_println!("We either ended at, or passed through zero. Incrementing counter");
            self.zero_passed += 1;
        }
        debug_println!("After the turn we're {:?}", self);
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    amount: usize,
}

fn part_two(instructions: &Vec<Instruction>) {
    let start = std::time::Instant::now();
    debug_println!("Creating lock");
    let mut lock = Lock::default();
    for instruction in instructions {
        lock.turn(instruction);
        debug_println!("#### Current Status: {:?}", lock);
    }
    println!("Result {}", lock.zero_passed);
    println!("Part Two took: {:?}", start.elapsed());
}

fn part_one(instructions: &Vec<Instruction>) {
    let start = std::time::Instant::now();
    debug_println!("Creating lock");
    let mut lock = Lock::default();
    for instruction in instructions {
        lock.turn(instruction);
        debug_println!("#### Current Status: {:?}", lock);
    }
    println!("Result {}", lock.zero_count);
    println!("Part One took: {:?}", start.elapsed());
}

fn parse_input(filename: &str) -> Vec<Instruction> {
    let instructions = read_file(filename)
        .iter()
        .map(|entry| {
            let (direction, amount_str) = entry.split_at(1);
            let amount: usize = amount_str.parse().unwrap();
            match direction {
                "L" => Instruction {
                    direction: Direction::Left,
                    amount,
                },
                "R" => Instruction {
                    direction: Direction::Right,
                    amount,
                },
                _ => panic!("Invalid instruction? {:?}", entry),
            }
        })
        .collect();
    instructions
}

fn main() {
    println!("Starting");
    let start = std::time::Instant::now();
    let instructions: Vec<Instruction> = parse_input("./data/day1.txt");
    debug_println!("Instructions: {:?}", instructions);
    part_one(&instructions);
    part_two(&instructions);
    part_two_brute(&instructions);
    println!("Overall time taken: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Lock::new(10, 0, 0), Instruction{ direction: Direction::Left, amount: 1}, 9)]
    #[case(Lock::new(10, 0, 0), Instruction{ direction: Direction::Left, amount: 2}, 8)]
    #[case(Lock::new(10, 0, 0), Instruction{ direction: Direction::Right, amount: 1}, 11)]
    #[case(Lock::new(10, 0, 0), Instruction{ direction: Direction::Right, amount: 2}, 12)]
    #[case(Lock::new(1, 0, 0), Instruction{ direction: Direction::Left, amount: 2}, 99)] // left past zero
    #[case(Lock::new(1, 0, 0), Instruction{ direction: Direction::Left, amount: 20}, 81)] // left past zero
    #[case(Lock::new(99, 0, 0), Instruction{ direction: Direction::Right, amount: 2}, 1)] // Right past 99
    #[case(Lock::new(99, 0, 0), Instruction{ direction: Direction::Right, amount: 20}, 19)] // Right past 99
    fn test_turn(
        #[case] lock: Lock,
        #[case] instruction: Instruction,
        #[case] final_location: usize,
    ) {
        let mut lock = lock.clone();
        lock.turn(&instruction);
        assert_eq!(lock.location, final_location);
    }

    #[rstest]
    fn test_count() {
        // Start out at 1 and move left to 0
        // Should increment counter
        let mut lock = Lock::new(1, 0, 0);
        assert_eq!(lock.zero_count, 0);
        lock.turn(&Instruction {
            direction: Direction::Left,
            amount: 1,
        });
        assert_eq!(lock.zero_count, 1);

        // Move left away from 0, and then right to get back to 0
        // Counter should increment
        lock.turn(&Instruction {
            direction: Direction::Left,
            amount: 1,
        });

        lock.turn(&Instruction {
            direction: Direction::Right,
            amount: 1,
        });

        assert_eq!(lock.zero_count, 2);
    }

    #[rstest]
    fn test_zero_passed() {
        let mut lock = Lock::new(1, 0, 0);
        lock.turn(&Instruction {
            direction: Direction::Left,
            amount: 2,
        });
        assert_eq!(lock.zero_passed, 1);
        lock.turn(&Instruction {
            direction: Direction::Right,
            amount: 2,
        });
        assert_eq!(lock.zero_passed, 2);
        lock.turn(&Instruction {
            direction: Direction::Right,
            amount: 100,
        });
        assert_eq!(lock.zero_passed, 3);
        lock.turn(&Instruction {
            direction: Direction::Left,
            amount: 700,
        });
        assert_eq!(lock.zero_passed, 10);
    }

    #[rstest]
    fn test_from_test_data() {
        let input = parse_input("./data/day1_test");
        let mut lock = Lock::default();
        for instruction in input {
            lock.turn(&instruction);
        }
        assert_eq!(lock.zero_count, 3);
        assert_eq!(lock.zero_passed, 6);
    }

    #[rstest]
    fn test_large_right() {
        let mut lock = Lock::new(50, 0, 0);
        lock.turn(&Instruction {
            direction: Direction::Right,
            amount: 1000,
        });
        assert_eq!(lock.zero_passed, 10);
    }

    #[rstest]
    fn test_start_zero_turn_left() {
        let mut lock = Lock::new(0, 0, 0);
        lock.turn(&Instruction {
            direction: Direction::Left,
            amount: 1,
        });
        assert_eq!(lock.zero_passed, 0);
    }

    #[rstest]
    fn test_start_zero_turn_right() {
        let mut lock = Lock::new(0, 0, 0);
        lock.turn(&Instruction {
            direction: Direction::Right,
            amount: 1,
        });
        assert_eq!(lock.zero_passed, 0);
    }
}
