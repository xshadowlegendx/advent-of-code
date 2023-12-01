
pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(calibrate_doc: &str) -> i32 {
        let mut answer = 0;

        let calibrates = calibrate_doc.split("\n");

        for cal in calibrates {
            let mut idx = 0;
            let mut tail_idx = cal.len();

            let mut heads = vec![];
            let mut tails = vec![];

            while idx != tail_idx {
                tail_idx -= 1;

                let head_char = cal.as_bytes()[idx];
                let tail_char = cal.as_bytes()[tail_idx];

                if head_char > 47 && head_char < 58 {
                    heads.push(head_char as char);
                }

                if tail_char > 47 && tail_char < 58 {
                    tails.push(tail_char as char);
                }

                if !heads.is_empty() && !tails.is_empty() {
                    break;
                }

                idx += 1;
            }

            let mut head = heads.first();
            let mut tail = tails.first();

            if head.is_some() && tail.is_none() {
                tail = heads.last();
            }

            if tail.is_some() && head.is_none() {
                head = tails.last();
            }

            let head = head.unwrap();
            let tail = tail.unwrap();

            println!("{cal} = {head}{tail}");

            answer += i32::from_str_radix(format!("{head}{tail}").as_str(), 10).unwrap();
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
    fn test_process() -> Result<(), PuzzleError> {
        let sample_test_case = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        assert_eq!(Puzzle::solve(sample_test_case), 142);

        Ok(())
    }
}
