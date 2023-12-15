
use std::ops::Rem;

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> usize {
        let mut answer = 0;

        for s in input.split(',') {
            let mut current_value: usize = 0;

            for ch in s.as_bytes() {
                current_value += *ch as usize;
                current_value *= 17;
                current_value = current_value.rem(256);
            }

            answer += current_value;
        }

        answer
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
        let sample_test_case = "HASH";

        assert_eq!(Puzzle::solve(sample_test_case), 52);

        let sample_test_case = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(Puzzle::solve(sample_test_case), 1320);

        Ok(())
    }
}
