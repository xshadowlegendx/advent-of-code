
use std::collections::{VecDeque, HashMap, HashSet};

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> usize {
        let mut beam_dir_switches = HashMap::<u8, HashMap<(i32, i32), Vec<(i32, i32)>>>::new();

        beam_dir_switches.insert(
            47,
            HashMap::from([
                ((0, 1), vec![(-1, 0)]),
                ((0, -1), vec![(1, 0)]),
                ((1, 0), vec![(0, -1)]),
                ((-1, 0), vec![(0, 1)]),
            ])
        );
        beam_dir_switches.insert(
            92,
            HashMap::from([
                ((0, 1), vec![(1, 0)]),
                ((0, -1), vec![(-1, 0)]),
                ((1, 0), vec![(0, 1)]),
                ((-1, 0), vec![(0, -1)]),
            ])
        );
        beam_dir_switches.insert(
            45,
            HashMap::from([
                ((1, 0), vec![(0, 1), (0, -1)]),
                ((-1, 0), vec![(0, 1), (0, -1)]),
            ])
        );
        beam_dir_switches.insert(
            124,
            HashMap::from([
                ((0, 1), vec![(1, 0), (-1, 0)]),
                ((0, -1), vec![(1, 0), (-1, 0)]),
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

        let mut distance_cache = HashMap::new();

        {
            let mut distance = 0;
            let mut prev_char = None;

            let mut r_idx: usize = 0;

            while r_idx < row_length as usize {
                let mut c_idx: usize = 0;

                while c_idx < col_length as usize {
                    if map[r_idx].as_bytes()[c_idx] != 46 || (c_idx > 0 && c_idx == col_length as usize - 1) {
                        if let Some((pr_idx, pc_idx)) = prev_char {
                            distance_cache.insert(((pr_idx, pc_idx), (0, 1)), ((r_idx, c_idx), distance));
                            distance_cache.insert(((r_idx, c_idx), (0, -1)), ((pr_idx, pc_idx), distance));
                        } else {
                            distance_cache.insert(((r_idx, 0), (0, 1)), ((r_idx, c_idx), distance));
                            distance_cache.insert(((r_idx, c_idx), (0, -1)), ((r_idx, 0), distance));
                        }

                        distance = 0;
                        prev_char = Some((r_idx, c_idx));
                    } else {
                        distance += 1;
                    }

                    c_idx += 1;
                }

                r_idx += 1;
            }

            let mut distance = 0;
            let mut prev_char = None;

            let mut c_idx: usize = 0;

            while c_idx < col_length as usize {
                let mut r_idx: usize = 0;

                while r_idx < row_length as usize {
                    if map[r_idx].as_bytes()[c_idx] != 46 || (r_idx > 0 && r_idx == row_length as usize - 1) {
                        if let Some((pr_idx, pc_idx)) = prev_char {
                            distance_cache.insert(((pr_idx, pc_idx), (1, 0)), ((r_idx, c_idx), distance));
                            distance_cache.insert(((r_idx, c_idx), (-1, 0)), ((pr_idx, pc_idx), distance));
                        } else {
                            distance_cache.insert(((0, c_idx), (1, 0)), ((r_idx, c_idx), distance));
                            distance_cache.insert(((r_idx, c_idx), (-1, 0)), ((0, c_idx), distance));
                        }

                        distance = 0;
                        prev_char = Some((r_idx, c_idx));
                    } else {
                        distance += 1;
                    }

                    r_idx += 1;
                }

                c_idx += 1;
            }
        }

        let mut beams = VecDeque::<(
            (i32, i32),
            (i32, i32),
            i32,
        )>::new();

        let mut beam_id = 0;

        let mut r_idx = 0;

        while r_idx < row_length {
            beams.push_back((
                (r_idx, -1),
                (0, 1),
                beam_id,
            ));
            beams.push_back((
                (r_idx, col_length),
                (0, -1),
                beam_id + 1,
            ));

            r_idx += 1;
            beam_id += 2;
        }

        let mut c_idx = 0;

        while c_idx < col_length {
            beams.push_back((
                (-1, c_idx),
                (1, 0),
                beam_id,
            ));
            beams.push_back((
                (row_length, c_idx),
                (-1, 0),
                beam_id + 1,
            ));

            c_idx += 1;
            beam_id += 2;
        }

        // beams.push_back((
        //     (-1, 3),
        //     (1, 0),
        //     0,
        // ));

        let mut global_energized = 0;

        let mut beam_state = HashMap::<i32, (HashMap<(i32, i32), (i32, i32)>, HashSet<((i32 ,i32), (i32, i32))>)>::new();

        while let Some((current_position, moving_direction, beam_id)) = beams.pop_front() {
            let next_row = current_position.0 + moving_direction.0;
            let next_col = current_position.1 + moving_direction.1;

            if !beam_state.contains_key(&beam_id) {
                beam_state.insert(beam_id, (HashMap::new(), HashSet::new()));
            }

            let (local_energized, local_visited) = beam_state
                .get_mut(&beam_id)
                .unwrap();

            if local_energized.len() > global_energized {
                global_energized = local_energized.len();
            }

            if next_row >= row_length || next_row < 0 || next_col >= col_length || next_col < 0 {
                continue;
            }

            local_energized.insert((next_row, next_col), moving_direction);

            let next_tile = &map[next_row as usize].as_bytes()[next_col as usize];

            if *next_tile != 46 {
                if local_visited.contains(&((next_row as i32, next_col as i32), moving_direction)) {
                    continue;
                }

                local_visited.insert(((next_row as i32, next_col as i32), moving_direction));

                // let distance  = distance_cache
                //     .get(&((next_row, next_col), moving_direction))
                //     .unwrap();

                // let mut idx = 0;

                // while idx < distance.1 {
                //     local_energized.insert((next_row as i32 + moving_direction.0, next_col as i32 + moving_direction.1), moving_direction);
                //     idx += 1;
                // }

                // next_row = distance.0.0;
                // next_col = distance.0.1;

                // next_tile = &map[next_row].as_bytes()[next_col];
            }

            if let Some(moves) = beam_dir_switches.get(next_tile) {
                if let Some(new_dir) = moves.get(&moving_direction) {
                    for dir in new_dir {
                        beams.push_back((
                            (next_row as i32, next_col as i32),
                            *dir,
                            beam_id
                        ));
                    }
                    continue;
                }
            }

            beams.push_back((
                (next_row as i32, next_col as i32),
                moving_direction,
                beam_id
            ));
        }

        global_energized
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

        assert_eq!(Puzzle::solve(sample_test_case), 51);

        Ok(())
    }
}
