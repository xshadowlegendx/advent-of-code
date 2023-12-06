
pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: Vec<Vec<i32>>) -> usize {
        let times = &input[0];
        let distances = &input[1];

        let mut race_idx = 0;
        let mut choices = vec![];

        let mut ans = 0;

        while race_idx < times.len() {
            let time = times[race_idx];
            let distance = distances[race_idx];

            let mut holds = 0;

            while holds < time {
                if (time - holds) * holds > distance {
                    choices.push(holds);
                }

                holds += 1;
            }

            if ans == 0 {
                ans = choices.len();
            } else if choices.len() > 0 {
                ans *= choices.len();
            }

            choices.clear();

            race_idx += 1;
        }

        ans
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
        let sample_test_case = vec![
            vec![7, 15, 30],
            vec![9, 40, 200],
        ];

        assert_eq!(Puzzle::solve(sample_test_case), 288);

        Ok(())
    }
}
