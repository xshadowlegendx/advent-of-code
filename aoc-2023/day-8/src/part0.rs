
use std::collections::VecDeque;

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut answers = 0;

        for l in input.split('\n') {
            let mut nums = VecDeque::new();

            for num in l.split(' ') {
                nums.push_back(i32::from_str_radix(num, 10).unwrap());
            }

            let mut prev_num = nums.pop_front().unwrap();

            let mut zeros = vec![];

            let mut length = nums.len();

            let mut length_count = 0;

            let mut last_vals = vec![];

            while let Some(num) = nums.pop_front() {
                length_count += 1;

                let diff = num - prev_num;
                
                if diff == 0 {
                    zeros.push(());
                }

                if zeros.len() == length {
                    last_vals.push(num);
                    break;
                }

                prev_num = num;
                nums.push_back(diff);

                if length_count == length {
                    prev_num = nums.pop_front().unwrap();
                    length -= 1;
                    length_count = 0;
                    zeros.clear();

                    last_vals.push(num);
                }
            }

            let mut history = 0;

            while let Some(val) = last_vals.pop() {
                let next_hist = history + val;

                history = next_hist;
            }

            answers += history;
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
        let sample_test_case = "0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45";

        assert_eq!(Puzzle::solve(sample_test_case), 114);

        let sample_test_case = "0 1 1 0";

        assert_eq!(Puzzle::solve(sample_test_case), -2);

        Ok(())
    }
}
