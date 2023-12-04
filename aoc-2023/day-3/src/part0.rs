
use std::collections::HashSet;

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut answers = 0;

        for line in input.split('\n') {
            let mut winners = HashSet::new();

            let parts = line
                .split('|')
                .collect::<Vec<&str>>();

            let winnings = parts[0]
                .split(':')
                .last()
                .unwrap()
                .split(char::is_whitespace)
                .filter(|n| !n.is_empty())
                .map(|n| {
                    i32::from_str_radix(n, 10).unwrap()
                })
                .collect::<Vec<i32>>();

            let my_numbers = parts[1]
                .split(char::is_whitespace)
                .filter(|n| !n.is_empty())
                .map(|n| {
                    i32::from_str_radix(n, 10).unwrap()
                })
                .collect::<Vec<i32>>();

            for num in winnings.iter() {
                for n in my_numbers.iter() {
                    if num == n {
                        winners.insert(num);
                    }
                }
            }

            if winners.len() > 0 {
                answers += 2_i32.pow((winners.len() - 1) as u32);
            }
        }

        answers
    }
}

#[derive(Debug)]
pub enum PuzzleError {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), PuzzleError> {
        let sample_test_case = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(Puzzle::solve(sample_test_case), 13);

        Ok(())
    }
}
