
use std::collections::HashMap;

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(calibrate_doc: &str) -> i32 {
        let mut answer = 0;

        let mut num_word_map = HashMap::new();

        num_word_map.insert("one", 1);
        num_word_map.insert("two", 2);
        num_word_map.insert("three", 3);
        num_word_map.insert("four", 4);
        num_word_map.insert("five", 5);
        num_word_map.insert("six", 6);
        num_word_map.insert("seven", 7);
        num_word_map.insert("eight", 8);
        num_word_map.insert("nine", 9);

        let mut num_word_start = HashMap::new();

        num_word_start.insert(111, vec!["one"]);
        num_word_start.insert(116, vec!["two", "three"]);
        num_word_start.insert(102, vec!["four", "five"]);
        num_word_start.insert(115, vec!["six", "seven"]);
        num_word_start.insert(101, vec!["eight"]);
        num_word_start.insert(110, vec!["nine"]);

        let calibrates = calibrate_doc.split("\n");

        for cal in calibrates {
            let mut idx = 0;

            let max_size = cal.len();

            let mut num_word = String::new();
            let mut nums = vec![];

            while idx < max_size {
                let c = cal.as_bytes()[idx];

                if c > 47 && c < 58 {
                    num_word.clear();
                    nums.push(String::from_utf8(vec![c]).unwrap());
                }

                if c > 96 {
                    if num_word_start.contains_key(&c) {
                        let mut iter_idx = 0;

                        while iter_idx < 5 && idx + iter_idx < max_size {
                            num_word.push(cal.as_bytes()[idx + iter_idx] as char);

                            if let Some(num_word_num) = num_word_map.get(num_word.as_str()) {
                                nums.push(num_word_num.to_string());
                            }

                            iter_idx += 1;
                        }
                    }

                    num_word.clear();
                }

                idx += 1;
            }

            let head = nums.first().unwrap();
            let tail = nums.last().unwrap();

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
    fn test_solve() -> Result<(), PuzzleError> {
        let sample_test_case = "two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen";

        assert_eq!(Puzzle::solve(sample_test_case), 281);

        Ok(())
    }
}
