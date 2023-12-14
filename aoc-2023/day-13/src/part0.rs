
pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut map = input
            .split('\n')
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let row_length = map.len();
        let col_length = map
            .first()
            .unwrap()
            .len();

        let mut col_idx = 0;

        while col_idx < col_length {
            let mut row_idx = 0;

            while row_idx < row_length {
                if map[row_idx].as_bytes()[col_idx] == 79 {
                    let mut go_up_idx = 1;

                    while row_idx as i32  - go_up_idx > -1 {
                        if map[row_idx - go_up_idx as usize].as_bytes()[col_idx] != 46 {
                            break;
                        }

                        go_up_idx += 1;
                    }

                    if go_up_idx > 1 {
                        go_up_idx -= 1;

                        let additional_move =
                            if map[row_idx - go_up_idx as usize].as_bytes()[col_idx] != 46 {
                                1
                            } else {
                                0
                            };

                        // debug
                        // for m in &map {
                        //     println!("{m}");
                        // }
                        // println!("before moving from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - go_up_idx as usize + additional_move);
                        // // println!("before moving from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - moves + additional_move);
                        // println!("*******");
                        // debug

                        unsafe {
                            *map
                                .get_mut(row_idx - go_up_idx as usize + additional_move)
                                // .get_mut(row_idx - moves + additional_move)
                                .unwrap()
                                .as_bytes_mut()
                                .get_mut(col_idx)
                                .unwrap() = 79;

                            *map
                                .get_mut(row_idx)
                                .unwrap()
                                .as_bytes_mut()
                                .get_mut(col_idx)
                                .unwrap() = 46;
                        }

                        // debug
                        // for m in &map {
                        //     println!("{m}");
                        // }
                        // println!("moved from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - go_up_idx as usize + additional_move);
                        // // println!("moved from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - moves + additional_move);
                        // println!("=======\n\n\n\n");
                        // debug
                    }
                }

                row_idx += 1;
            }

            col_idx += 1;
        }

        let mut reversed_count = row_length as i32;

        map
            .iter()
            .fold(0, |acc, l| {
                let acc = acc + l.matches('O').count() as i32 * reversed_count;

                reversed_count -= 1;

                acc
            })
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
        let sample_test_case = "O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....";

        assert_eq!(Puzzle::solve(sample_test_case), 136);

        Ok(())
    }
}
