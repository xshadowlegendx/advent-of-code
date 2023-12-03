use std::collections::HashSet;


pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut answers = 0;
        let mut outer_idx: i32 = 0;

        let lines = input
            .split('\n')
            .collect::<Vec<&str>>();

        let all_max = lines.len();

        for line in &lines {
            let mut idx = 0;
            let line = line.as_bytes();
            let max_size = line.len();

            while idx < max_size {
                let c = line[idx];

                if c == 42 {
                    let checks = vec![
                        (outer_idx, idx as i32 + 1), // right
                        (outer_idx, idx as i32 - 1), // left
                        (outer_idx - 1, idx as i32), // up
                        (outer_idx + 1, idx as i32), // down
                        (outer_idx + 1, idx as i32 - 1), // bottom left
                        (outer_idx + 1, idx as i32 + 1), // bottom right
                        (outer_idx - 1, idx as i32 - 1), // top left
                        (outer_idx - 1, idx as i32 + 1), // top right
                    ];

                    let mut mults = vec![];
                    let mut neighbors = HashSet::new();

                    for (row_idx, row_col_idx) in checks {
                        if row_idx < 0 || row_idx > all_max as i32 || row_col_idx < 0 || row_col_idx > max_size as i32 {
                            continue;
                        }

                        let l = lines[row_idx as usize].as_bytes();

                        if l[row_col_idx as usize] > 47 && l[row_col_idx as usize] < 58 {
                            if !neighbors.contains(&(row_idx, row_col_idx)) {
                                mults.push((row_idx, row_col_idx));

                                neighbors.insert((row_idx, row_col_idx));
                                neighbors.insert((row_idx, row_col_idx - 1));
                                neighbors.insert((row_idx, row_col_idx + 1));
                            }
                        }

                        if mults.len() == 2 {
                            break;
                        }
                    }

                    if mults.len() == 2 {
                        let mut prod = 1;

                        for (row_idx, row_col_idx) in mults {
                            let line = lines[row_idx as usize].as_bytes();

                            let mut num_construct = String::new();

                            num_construct.push(line[row_col_idx as usize] as char);

                            let mut idx = 1;

                            let mut is_check_left = true;
                            let mut is_check_right = true;

                            while is_check_left || is_check_right {
                                if is_check_right && row_col_idx + idx < max_size as i32 {
                                    let col_idx = (row_col_idx + idx) as usize;

                                    if line[col_idx] > 47 && line[col_idx] < 58 {
                                        num_construct.push(line[col_idx] as char);
                                    } else {
                                        is_check_right = false;
                                    }
                                } else {
                                    is_check_right = false;
                                }

                                if is_check_left && row_col_idx - idx >= 0 {
                                    let col_idx = (row_col_idx - idx) as usize;

                                    if line[col_idx] > 47 && line[col_idx] < 58 {
                                        num_construct.insert(0, line[col_idx] as char);
                                    } else {
                                        is_check_left = false;
                                    }
                                } else {
                                    is_check_left = false;
                                }

                                idx += 1;
                            }

                            prod *= i32::from_str_radix(&num_construct, 10).unwrap();
                        }

                        answers += prod;
                    }
                }

                idx += 1;
            }

            outer_idx += 1;
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
        let sample_test_case = "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

        assert_eq!(Puzzle::solve(sample_test_case), 467835);

        Ok(())
    }
}
