
pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> usize {
        let mut maps = vec![];

        maps.push(vec![]);

        for l in input.split('\n') {
            if l.is_empty() {
                maps.push(vec![]);
                continue;
            }

            maps
                .last_mut()
                .unwrap()
                .push(l);
        }

        let mut cols_count = 0;
        let mut rows_count = 0;
        let mut map_idx = 0;

        while map_idx < maps.len() {
            let map = &maps[map_idx];

            // debug

            let mut d_idx = 0;
            for m in map {
                println!("{m} ({d_idx})");
                d_idx += 1;
            }
            for idx in 0..map[0].as_bytes().len() {
                print!("{idx},");
            }
            println!("");
            println!("&&&&&&&");

            // debug

            map_idx += 1;

            let row_length = map.len();
            let col_length = map
                .first()
                .unwrap()
                .len();

            // vertical
            let mut col_idx = 0;
            let mut max_col = -1;

            while col_idx < col_length {
                let mut row_idx = 0;

                let mut match_row_count = 0;

                while row_idx < row_length && col_idx + 1 < col_length {
                    if map[row_idx].as_bytes()[col_idx] == map[row_idx].as_bytes()[col_idx + 1] {
                        match_row_count += 1;
                    }

                    row_idx += 1;
                }

                if match_row_count == row_length {
                    let mut is_break = false;
                    let mut iter_idx = 1;

                    println!("vertical found: {:?}", col_idx);

                    while col_idx as i32 - iter_idx >= 0 && col_idx + (iter_idx as usize) + 1 < col_length {
                        let mut row_idx = 0;

                        while row_idx < row_length {
                            if map[row_idx].as_bytes()[col_idx - iter_idx as usize] != map[row_idx].as_bytes()[col_idx + iter_idx as usize + 1] {
                                println!("breaking: ({},{}) and ({}, {})", row_idx, col_idx - iter_idx as usize, row_idx, col_idx + iter_idx as usize + 1);
                                is_break = true;
                                break;
                            }

                            row_idx += 1;
                        }

                        if is_break {
                            break;
                        }

                        iter_idx += 1;
                    }

                    if !is_break {
                        if col_idx as i32 > max_col {
                            max_col = col_idx as i32;
                        }
                    }
                }

                col_idx += 1;
            }

            let local_col_counts =
                if max_col > -1 {
                    max_col as usize + 1
                } else {
                    0
                };

            // horizontal
            let mut row_idx = 0;
            let mut max_row = -1;

            while row_idx < row_length {
                if row_idx + 1 < row_length && map[row_idx] == map[row_idx + 1] {
                    println!("horizontal found: {:?}", row_idx);

                    let mut is_break = false;
                    let mut iter_idx = 1;

                    while row_idx as i32 - iter_idx >= 0 && row_idx + (iter_idx as usize) + 1 < row_length {
                        if map[row_idx - iter_idx as usize] != map[row_idx + iter_idx as usize + 1] {
                            println!("breaking: ({},{}) and ({}, {})", row_idx - iter_idx as usize, col_idx, row_idx + iter_idx as usize + 1, col_idx);
                            is_break = true;
                            break;
                        }

                        if is_break {
                            break;
                        }

                        iter_idx += 1;
                    }

                    if !is_break {
                        if row_idx as i32 > max_row {
                            max_row = row_idx as i32;
                        }
                        break;
                    }
                }

                row_idx += 1;
            }

            let local_row_counts =
                if max_row > -1 {
                    max_row as usize + 1
                } else {
                    0
                };

            println!("vertical: {}, horizontal: {}", local_col_counts, local_row_counts);
            println!("{},{}", cols_count, rows_count);

            if local_col_counts > local_row_counts {
                cols_count += local_col_counts;
            } else {
                rows_count += local_row_counts;
            }

            println!("{},{}", cols_count, rows_count);
            println!("==========");
        }

        cols_count + rows_count * 100
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
        let sample_test_case = "#.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
        \n\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#";

        assert_eq!(Puzzle::solve(sample_test_case), 405);

        Ok(())
    }
}
