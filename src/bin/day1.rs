#[derive(Debug, Clone, Copy)]
struct Lock {
    location: usize,
    zero_count: usize,
}

impl Default for Lock {
    fn default() -> Self {
        Lock::new(0, 0)
    }
}

enum Direction {
    Left,
    Right,
}

impl Lock {
    fn new(location: usize, zero_count: usize) -> Self {
        Self {
            location,
            zero_count,
        }
    }

    fn turn(&mut self, direction: Direction, amount: usize) {
        match direction {
            Direction::Left => {
                log::info!("Turning Left {}", amount);
                // Have it roll past 0
                if self.location < amount {
                    log::info!(
                        "{} < {}, so we increase location by 100",
                        self.location,
                        amount
                    );
                    self.location += 100;
                    log::info!("After increment, location = {}", self.location);
                }
                self.location -= amount;
                log::info!("After moving left by {} we're at {}", amount, self.location);
                if self.location == 0 {
                    self.zero_count += 1;
                }
            }
            Direction::Right => {
                self.location += amount;
                // check if we've rolled past 99
                if self.location > 99 {
                    self.location -= 100;
                }
            }
        }
    }
}

fn main() {
    let lock = Lock::default();
    println!("{:?}", lock);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[rstest]
    #[case(Lock::new(10, 0), Direction::Left, 1, 9)]
    #[case(Lock::new(10, 0), Direction::Left, 2, 8)]
    #[case(Lock::new(10, 0), Direction::Right, 1, 11)]
    #[case(Lock::new(10, 0), Direction::Right, 2, 12)]
    #[case(Lock::new(1, 0), Direction::Left, 2, 99)] // left past zero
    #[case(Lock::new(1, 0), Direction::Left, 20, 81)] // left past zero
    #[case(Lock::new(99, 0), Direction::Right, 2, 1)] // Right past 99
    #[case(Lock::new(99, 0), Direction::Right, 20, 19)] // Right past 99
    fn test_turn(
        #[case] lock: Lock,
        #[case] direction: Direction,
        #[case] amount: usize,
        #[case] final_location: usize,
    ) {
        let mut lock = lock.clone();
        lock.turn(direction, amount);
        assert_eq!(lock.location, final_location);
    }
}
