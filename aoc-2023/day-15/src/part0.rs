
use std::collections::{VecDeque, HashMap};

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> usize {
        let mut energized = HashMap::new();

        let mut beam_dir_switches = HashMap::<u8, HashMap<(i32, i32), (i32, i32)>>::new();

        beam_dir_switches.insert(
            47,
            HashMap::from([
                ((0, 1), (-1, 0)),
                ((0, -1), (1, 0)),
                ((1, 0), (0, -1)),
                ((-1, 0), (0, 1)),
            ])
        );
        beam_dir_switches.insert(
            92,
            HashMap::from([
                ((0, 1), (1, 0)),
                ((0, -1), (-1, 0)),
                ((1, 0), (0, 1)),
                ((-1, 0), (0, -1)),
            ])
        );

        let map = input
            .split('\n')
            .map(|f| f.to_string())
            .collect::<Vec<String>>();

        let row_length = map.len() as i32;
        let col_length = map
            .first()
            .unwrap()
            .len() as i32;

        let mut beams = VecDeque::<((i32, i32), (i32, i32))>::from([((0, -1), (0, 1))]);

        let mut prev_energized_length = 0;
        let mut is_not_moving = false;
        let mut is_not_moving_count = 0;

        while let Some((current_position, moving_direction)) = beams.pop_front() {
            let next_row = current_position.0 + moving_direction.0;
            let next_col = current_position.1 + moving_direction.1;

            if next_row >= row_length || next_row < 0 || next_col >= col_length || next_col < 0 {
                continue;
            }

            energized.insert((next_row, next_col), moving_direction);

            if energized.len() == prev_energized_length {
                if is_not_moving {
                    is_not_moving = true;
                }

                is_not_moving_count += 1;
            } else {
                is_not_moving = false;
                is_not_moving_count = 0;
            }

            if is_not_moving_count > row_length * col_length * 1024 {
                break;
            }

            prev_energized_length = energized.len();

            let next_row = next_row as usize;
            let next_col = next_col as usize;

            let next_tile = &map[next_row].as_bytes()[next_col];

            // debug
            // let mut r_idx = 0;
            // while r_idx < row_length {
            //     let mut c_idx = 0;
            //     while c_idx < col_length {
            //         if let Some(move_dir) = energized.get(&(r_idx, c_idx)) {
            //             if *&map[r_idx as usize].as_bytes()[c_idx as usize] == 46 {
            //                 match move_dir {
            //                     (0, 1) => print!(">"),
            //                     (0, -1) => print!("<"),
            //                     (1, 0) => print!("v"),
            //                     _ => print!("^"),
            //                 }
            //             } else {
            //                 print!("{}", *&map[r_idx as usize].as_bytes()[c_idx as usize] as char);
            //             }
            //         } else {
            //             print!("{}", *&map[r_idx as usize].as_bytes()[c_idx as usize] as char);
            //         }

            //         c_idx += 1;
            //     }
            //     println!("");
            //     r_idx += 1;
            // }
            // println!("=======\n\n");
            // debug

            match *next_tile {
                47 | 92 => {
                    let moves = beam_dir_switches
                        .get(next_tile)
                        .unwrap();

                    let new_dir = moves
                        .get(&moving_direction)
                        .unwrap();

                    // println!("{:?}", new_dir);
                    // println!("==== new dir =====");

                    beams.push_back((
                        (next_row as i32, next_col as i32),
                        *new_dir
                    ));
                    continue;
                },
                45 => {
                    if moving_direction != (0, 1) && moving_direction != (0, -1) {
                        beams.push_back((
                            (next_row as i32, next_col as i32),
                            (0, 1),
                        ));
                        beams.push_back((
                            (next_row as i32, next_col as i32),
                            (0, -1),
                        ));

                        continue;
                    }
                },
                124 => {
                    if moving_direction != (1, 0) && moving_direction != (-1, 0) {
                        beams.push_back((
                            (next_row as i32, next_col as i32),
                            (1, 0),
                        ));
                        beams.push_back((
                            (next_row as i32, next_col as i32),
                            (-1, 0),
                        ));

                        continue;
                    }
                },
                _ => {},
            }

            beams.push_back((
                (next_row as i32, next_col as i32),
                moving_direction
            ));
        }

        energized.len()
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
        let sample_test_case = ".|...\\....\n\
        |.-.\\.....\n\
        .....|-...\n\
        ........|.\n\
        ..........\n\
        .........\\\n\
        ..../.\\\\..\n\
        .-.-/..|..\n\
        .|....-|.\\\n\
        ..//.|....";

        assert_eq!(Puzzle::solve(sample_test_case), 46);

        Ok(())
    }
}
