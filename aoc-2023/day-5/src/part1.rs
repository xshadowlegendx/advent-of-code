
pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: Vec<i64>) -> usize {
        let mut choices = vec![];

        let mut ans = 0;

        let time = input[0];
        let distance = input[1];

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
            71530,
            940200,
        ];

        assert_eq!(Puzzle::solve(sample_test_case), 71503);

        Ok(())
    }
}
