
use std::collections::{VecDeque, HashMap};

pub struct Puzzle;

impl Puzzle {
    // #[tracing::instrument]
    // fn generate_combination(combs: &mut VecDeque<String>, memoize: &mut HashMap<String, i32>) -> i32 {
    //     let mut counted = 0;

    //     while let Some(comb) = combs.pop_front() {
    //         if let Some(comb_count) = memoize.get(&comb) {
    //             counted += comb_count;
    //             continue;
    //         }

    //         let mut idx = 0;
    //         let len = comb.len();

    //         let mut should_generate = false;

    //         while idx < len {
    //             if comb.as_bytes()[idx] == 63 {
    //                 for choice in [46, 35] {
    //                     let mut new_comb = comb.to_string();
    //                     unsafe {
    //                         new_comb.as_bytes_mut()[idx] = choice;
    //                     }
    //                     combs.push_front(new_comb);

    //                     should_generate = true;
    //                 }
    //                 break;
    //             }

    //             idx += 1;
    //         }

    //         if should_generate {
    //             let comb_count = Puzzle::generate_combination(combs, memoize);

    //             return comb_count;
    //         }

    //         if let Some(is_count) = memoize.get(&comb) {
    //             if *is_count && !ans.contains_key(&comb) {
    //                 answers += 1;
    //                 ans.insert(comb, answers);
    //             }

    //             continue;
    //         }

    //         let mut failure_count = 0;
    //         let mut count_start = None;

    //         let mut f_counted = vec![];

    //         for ch in comb.chars() {
    //             if let Some(start) = count_start {
    //                 if ch != start {
    //                     if failure_count > 0 {
    //                         f_counted.push(failure_count);
    //                     }

    //                     failure_count = 0;
    //                 }
    //             }

    //             count_start = Some(ch);

    //             if ch == '.' {
    //                 continue;
    //             }

    //             failure_count += 1;
    //         }

    //         if failure_count > 0 {
    //             f_counted.push(failure_count);
    //         }

    //         if ans.contains_key(&comb) {
    //             continue;
    //         }

    //         if f_counted == ctn_f {
    //             memoize.insert(comb.clone(), true);
    //             ans.insert(comb, answers + 1);
    //             answers += 1;
    //             continue;
    //         }
    //     }

    //     counted
    // }

    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut answers = 0;

        for l in input.split('\n') {
            let record = l
                .split(' ')
                .collect::<Vec<&str>>();

            let mut ctn_f = vec![];

            for _ in 0..5 {
                ctn_f.append(&mut record[1]
                    .split(',')
                    .map(|v| i32::from_str_radix(v, 10).unwrap())
                    .collect::<Vec<i32>>()
                );
            }

            let mut first_comb = String::new();

            let mut combinations = VecDeque::new();

            for _ in 0..5 {
                first_comb.push(',');
                first_comb.push_str(record[0]);
            }

            combinations.push_front(first_comb);

            let mut memoize = HashMap::new();

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
                            combinations.push_front(new_comb);
                        }

                        break;
                    }

                    idx += 1;
                }

                if idx != len {
                    continue;
                }

                if let Some(is_count) = memoize.get(&comb) {
                    if *is_count && !ans.contains_key(&comb) {
                        answers += 1;
                        ans.insert(comb, answers);
                    }

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
                    memoize.insert(comb.clone(), true);
                    ans.insert(comb, answers + 1);
                    answers += 1;
                    continue;
                }

                memoize.insert(comb, false);
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

        // assert_eq!(Puzzle::solve(sample_test_case), 525152);

        Ok(())
    }
}
