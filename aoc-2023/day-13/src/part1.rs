
use std::collections::{HashMap, HashSet};

pub struct Puzzle;

impl Puzzle {
    fn go_right(map: &mut Vec<String>) {
        let row_length = map.len();
        let col_length = map
            .first()
            .unwrap()
            .len();

        let mut row_idx = 0;

        while row_idx < row_length {
            let mut col_idx = col_length as i32 - 1;

            while col_idx > -1 {
                if map[row_idx].as_bytes()[col_idx as usize] == 79 {
                    let col_idx = col_idx as usize;

                    let mut move_idx = 1;

                    while col_idx + move_idx < col_length {
                        if map[row_idx].as_bytes()[col_idx + move_idx] != 46 {
                            break;
                        }

                        move_idx += 1;
                    }

                    if move_idx > 1 {
                        move_idx -= 1;

                        let additional_move =
                            if map[row_idx].as_bytes()[col_idx + move_idx] != 46 {
                                1
                            } else {
                                0
                            };                       

                        unsafe {
                            *map
                                .get_mut(row_idx)
                                .unwrap()
                                .as_bytes_mut()
                                .get_mut(col_idx + move_idx as usize + additional_move)
                                .unwrap() = 79;

                            *map
                                .get_mut(row_idx)
                                .unwrap()
                                .as_bytes_mut()
                                .get_mut(col_idx)
                                .unwrap() = 46;
                        }
                    }
                }

                col_idx -= 1;
            }

            row_idx += 1;
        }
    }

    fn go_left(map: &mut Vec<String>) {
        let row_length = map.len();
        let col_length = map
            .first()
            .unwrap()
            .len();

        let mut row_idx = 0;

        while row_idx < row_length {
            let mut col_idx = 0;

            while col_idx < col_length {
                if map[row_idx].as_bytes()[col_idx] == 79 {
                    let mut move_idx = 1;

                    while col_idx as i32 - move_idx > -1 {
                        if map[row_idx].as_bytes()[col_idx - move_idx as usize] != 46 {
                            break;
                        }

                        move_idx += 1;
                    }

                    if move_idx > 1 {
                        move_idx -= 1;

                        let additional_move =
                            if map[row_idx].as_bytes()[col_idx - move_idx as usize] != 46 {
                                1
                            } else {
                                0
                            };                       

                        unsafe {
                            *map
                                .get_mut(row_idx)
                                .unwrap()
                                .as_bytes_mut()
                                .get_mut(col_idx - move_idx as usize + additional_move)
                                .unwrap() = 79;

                            *map
                                .get_mut(row_idx)
                                .unwrap()
                                .as_bytes_mut()
                                .get_mut(col_idx)
                                .unwrap() = 46;
                        }
                    }
                }

                col_idx += 1;
            }

            row_idx += 1;
        }
    }

    fn go_down(map: &mut Vec<String>) {
        let row_length = map.len();
        let col_length = map
            .first()
            .unwrap()
            .len();

        let mut col_idx = 0;

        while col_idx < col_length {
            let mut row_idx = row_length as i32 - 1;

            while row_idx > -1 {
                if map[row_idx as usize].as_bytes()[col_idx] == 79 {
                    let row_idx = row_idx as usize;

                    let mut move_idx = 1;

                    while row_idx + move_idx < row_length {
                        if map[row_idx + move_idx as usize].as_bytes()[col_idx] != 46 {
                            break;
                        }

                        move_idx += 1;
                    }

                    if move_idx > 1 {
                        move_idx -= 1;

                        let additional_move =
                            if map[row_idx + move_idx as usize].as_bytes()[col_idx] != 46 {
                                1
                            } else {
                                0
                            };

                        unsafe {
                            *map
                                .get_mut(row_idx + move_idx as usize + additional_move)
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
                        // for m in map.iter() {
                        //     println!("{m}");
                        // }
                        // println!("moved from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx + move_idx as usize + additional_move);
                        // // println!("moved from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - moves + additional_move);
                        // println!("********");
                        // debug
                    }
                }

                row_idx -= 1;
            }

            col_idx += 1;
        }
    }

    #[tracing::instrument]
    fn go_up(map: &mut Vec<String>) {
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
                    let mut move_idx = 1;

                    while row_idx as i32 - move_idx > -1 {
                        if map[row_idx - move_idx as usize].as_bytes()[col_idx] != 46 {
                            break;
                        }

                        move_idx += 1;
                    }

                    if move_idx > 1 {
                        move_idx -= 1;

                        let additional_move =
                            if map[row_idx - move_idx as usize].as_bytes()[col_idx] != 46 {
                                1
                            } else {
                                0
                            };

                        // debug
                        // for m in &map {
                        //     println!("{m}");
                        // }
                        // println!("before moving from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - move_idx as usize + additional_move);
                        // // println!("before moving from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - moves + additional_move);
                        // println!("*******");
                        // debug

                        unsafe {
                            *map
                                .get_mut(row_idx - move_idx as usize + additional_move)
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
                        // for m in map.iter() {
                        //     println!("{m}");
                        // }
                        // println!("moved from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - move_idx as usize + additional_move);
                        // // println!("moved from: ({row_idx},{col_idx}) to ({},{col_idx})", row_idx - moves + additional_move);
                        // println!("=======\n\n\n\n");
                        // debug
                    }
                }

                row_idx += 1;
            }

            col_idx += 1;
        }
    }

    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut map = input
            .split('\n')
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let row_length = map.len();

        // let mut cycle_end = 0;
        // let mut first_cycle_sum = 0;

        // let mut is_cycle_resetting = false;
        // let mut cycle_reset_count = 0;
        // let mut previous_cycle_length = 0;
        let mut cycles = 1000000000;
        let mut previous_save_length = 0;

        // let mut possible_ans = BTreeMap::new();
        let mut answer = 0;
        let mut possible_ans = vec![];
        let mut saves = HashMap::new();

        while cycles > 0 {
            Puzzle::go_up(&mut map);

            // debug
            // for m in map.iter() {
            //     println!("{m}");
            // }
            // println!("=======\n\n");
            // debug

            Puzzle::go_left(&mut map);

            // debug
            // for m in map.iter() {
            //     println!("{m}");
            // }
            // println!("=======\n\n");
            // debug

            Puzzle::go_down(&mut map);

            // debug
            // for m in map.iter() {
            //     println!("{m}");
            // }
            // println!("=======\n\n");
            // debug

            Puzzle::go_right(&mut map);

            // debug
            // for m in map.iter() {
            //     println!("{m}");
            // }
            // println!("=======\n\n");
            // debug

            let mut reversed_count = row_length as i32;

            let sum =
                map
                    .iter()
                    .fold(0, |acc, l| {
                        let acc = acc + l.matches('O').count() as i32 * reversed_count;

                        reversed_count -= 1;

                        acc
                    });

            saves.insert(map.clone(), sum);

            if previous_save_length == saves.len() {
                let mut pos_count = 0;
                let mut pos_ans = HashSet::new();
                let mut possible_answers = vec![];

                let mut cycle_checked = 0;

                while let Some(val) = saves.get(&map) {
                    if pos_count > pos_ans.len() {
                        // pos_ans.insert(map.clone());
                        possible_answers.pop();
                        break;
                    }

                    Puzzle::go_up(&mut map);
                    Puzzle::go_left(&mut map);
                    Puzzle::go_down(&mut map);
                    Puzzle::go_right(&mut map);

                    pos_count += 1;
                    pos_ans.insert(map.clone());

                    cycle_checked += 1;

                    possible_answers.push(val);
                }

                answer = *possible_answers[(cycles - cycle_checked) % possible_answers.len()];

                break;
            } else {
                possible_ans.push(sum);
                previous_save_length = saves.len();
            }

            cycles -= 1;
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

        assert_eq!(Puzzle::solve(sample_test_case), 64);

        Ok(())
    }
}
