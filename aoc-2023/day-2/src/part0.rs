
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
            let mut start_idx = 0;
            let mut num_construct = String::new();
            
            let mut idx = 0;
            let line = line.as_bytes();
            let max_size = line.len();

            while idx < max_size {
                let c = line[idx];

                if c > 47 && c < 58 {
                    if num_construct.is_empty() {
                        start_idx = idx;
                    }

                    num_construct.push(c as char);

                    if idx != max_size - 1 {
                        idx += 1;
                        continue;
                    }
                }

                if !num_construct.is_empty() {
                    let mut checks: Vec<(i32, i32)> = vec![
                        (outer_idx, start_idx as i32 - 1), // left
                        (outer_idx, idx as i32), // right
                        (outer_idx + 1, start_idx as i32), // down left
                        (outer_idx - 1, start_idx as i32), // up left
                        (outer_idx + 1, start_idx as i32 - 1), // diag left down
                        (outer_idx - 1, start_idx as i32 - 1), // diag left up
                        (outer_idx - 1, idx as i32), // diag left up
                        (outer_idx + 1, idx as i32), // diag right up
                        (outer_idx - 1, idx as i32 - 1), // up right
                        (outer_idx + 1, idx as i32 - 1), // down right
                    ];

                    for mid in start_idx..idx {
                        checks.push((outer_idx + 1, mid as i32));
                        checks.push((outer_idx - 1, mid as i32));
                    }

                    for (row_idx, row_col_idx) in checks {
                        if row_idx >= 0 && row_idx < all_max as i32 {
                            let l = lines[row_idx as usize].as_bytes();

                            if row_col_idx < 0 || row_col_idx > l.len() as i32 {
                                continue;
                            }

                            if l[row_col_idx as usize] != 46 && (
                                (l[row_col_idx as usize] > 32 && l[row_col_idx as usize] < 48) ||
                                (l[row_col_idx as usize] > 57 && l[row_col_idx as usize] < 65) ||
                                (l[row_col_idx as usize] > 90 && l[row_col_idx as usize] < 97) ||
                                (l[row_col_idx as usize] > 122 && l[row_col_idx as usize] < 127)
                            ) {
                                answers += i32::from_str_radix(&num_construct, 10).unwrap();
                                break;
                            }
                        }
                    }
                }

                num_construct.clear();

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

        assert_eq!(Puzzle::solve(sample_test_case), 4361);

        Ok(())
    }
}
