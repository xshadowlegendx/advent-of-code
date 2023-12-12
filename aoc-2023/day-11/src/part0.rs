
use std::collections::{VecDeque, HashMap};

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut answers = 0;

        for l in input.split('\n') {
            let record = l
                .split(' ')
                .collect::<Vec<&str>>();

            let ctn_f = record[1]
                .split(',')
                .map(|v| i32::from_str_radix(v, 10).unwrap())
                .collect::<Vec<i32>>();

            let mut combinations = VecDeque::new();

            combinations.push_back(record[0].to_string());

            let mut ans = HashMap::new();

            while let Some(comb) = combinations.pop_front() {
                let mut idx = 0;
                let len = comb.len();

                while idx < len {
                    if comb.as_bytes()[idx] == 63 {
                        for choice in [46, 35] {
                            let mut new_comb = comb.to_string();
                            unsafe {
                                new_comb.as_bytes_mut()[idx] = choice;
                            }
                            combinations.push_back(new_comb);
                        }

                        break;
                    }

                    idx += 1;
                }

                if idx != len {
                    continue;
                }

                let mut failure_count = 0;
                let mut count_start = None;

                let mut f_counted = vec![];

                for ch in comb.chars() {
                    if let Some(start) = count_start {
                        if ch != start {
                            if failure_count > 0 {
                                f_counted.push(failure_count);
                            }

                            failure_count = 0;
                        }
                    }

                    count_start = Some(ch);

                    if ch == '.' {
                        continue;
                    }

                    failure_count += 1;
                }

                if failure_count > 0 {
                    f_counted.push(failure_count);
                }

                if ans.contains_key(&comb) {
                    continue;
                }

                if f_counted == ctn_f {
                    ans.insert(comb, answers + 1);
                    answers += 1;
                }
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
        let sample_test_case = "???.### 1,1,3\n\
            .??..??...?##. 1,1,3\n\
            ?#?#?#?#?#?#?#? 1,3,1,6\n\
            ????.#...#... 4,1,1\n\
            ????.######..#####. 1,6,5\n\
            ?###???????? 3,2,1";

        assert_eq!(Puzzle::solve(sample_test_case), 21);

        Ok(())
    }
}
