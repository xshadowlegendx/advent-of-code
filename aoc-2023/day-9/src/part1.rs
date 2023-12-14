
use std::collections::{VecDeque, BTreeMap, HashMap};

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let maps = input
            .split('\n')
            .collect::<Vec<&str>>();

        let row_length = maps.len();
        let col_length = maps[0].len();

        let mut row_idx = 0;

        let mut travels = VecDeque::new();

        let mut visited = HashMap::new();

        while row_idx < row_length {
            let mut col_idx = 0;

            while col_idx < col_length {
                if maps[row_idx].as_bytes()[col_idx] == 83 {
                    visited.insert(
                        (row_idx, col_idx),
                        (maps[row_idx].as_bytes()[col_idx] as char, 0, (row_idx, col_idx))
                    );

                    travels.push_back((row_idx, col_idx));

                    break;
                }

                col_idx += 1;
            }

            if !travels.is_empty() {
                break;
            }

            row_idx += 1;
        }

        let mut moves = BTreeMap::<u8, Vec<(i32, i32)>>::new();

        moves.insert(124, vec![(-1, 0), (1, 0)]); // | north south
        moves.insert(45, vec![(0, -1), (0, 1)]); // - west east
        moves.insert(76, vec![(-1, 0), (0, 1)]); // L - north east
        moves.insert(74, vec![(-1, 0), (0, -1)]); // J - north west
        moves.insert(70, vec![(1, 0), (0, 1)]); // F - south east
        moves.insert(55, vec![(1, 0), (0, -1)]); // 7 - south west

        let checks: [(i32, i32); 4] = [
            (0, 1), // right
            (1, 0), // down
            (0, -1), // left
            (-1, 0), // up
        ];

        // let mut loops = HashMap::new();

        while let Some((row_idx, col_idx)) = travels.pop_front() {
            // debug

            // let mut d = vec![vec!['.'; col_length]; row_length];

            // let mut m = HashMap::new();

            // m.insert('S', '*');
            // m.insert('-', '-');
            // m.insert('|', '|');
            // m.insert('F', '┌');
            // m.insert('7', '┐');
            // m.insert('L', '└');
            // m.insert('J', '┘');

            // for ((r, c), (ch, _, _)) in &visited {
            //     d[*r][*c] = *m.get(&ch).unwrap();
            // }

            // for dd in d {
            //     for ddd in dd {
            //         print!("{ddd}");
            //     }
            //     println!("");
            // }

            // println!("========");

            // debug

            let current = maps[row_idx].as_bytes()[col_idx];

            let has_to_move = moves.get(&current);

            for (r, c) in checks {
                if row_idx as i32 + r < 0 || row_idx as i32 + r >= row_length as i32 {
                    continue;
                }

                if col_idx as i32 + c < 0 || col_idx as i32 + c >= col_length as i32 {
                    continue;
                }

                if let Some(mv) = has_to_move {
                    if !mv.contains(&(r, c)) {
                        continue;
                    }
                }

                let next_pos_idx = ((row_idx as i32 + r) as usize, (col_idx as i32 + c) as usize);

                let m = maps[next_pos_idx.0].as_bytes()[next_pos_idx.1];

                if m == 46 {
                    continue;
                }

                let v = visited.get(&(row_idx, col_idx)).unwrap();

                if v.2 == next_pos_idx {
                    continue;
                }

                if let Some(mv) = moves.get(&m) {
                    let mut is_connected = false;

                    for (r, c) in mv {
                        if next_pos_idx.0 as i32 + r == row_idx as i32 && next_pos_idx.1 as i32 + c == col_idx as i32 {
                            is_connected = true;
                        }
                    }

                    if is_connected {
                        if visited.contains_key(&next_pos_idx) {
                            continue;
                        }

                        visited.insert(next_pos_idx, (m as char, v.1 + 1, (row_idx, col_idx)));
                        travels.push_back(next_pos_idx);
                    }
                }
            }
        }

        let mut d = vec![vec!['.'; col_length]; row_length];

        let mut m = HashMap::new();

        m.insert('S', '*');
        m.insert('-', '-');
        m.insert('|', '|');
        m.insert('F', '┌');
        m.insert('7', '┐');
        m.insert('L', '└');
        m.insert('J', '┘');
        
        for ((r, c), (ch, _, _)) in visited {
            d[r][c] = *m.get(&ch).unwrap();
        }

        let mut enclose_tile_count = 0;

        let row_length = d.len();
        let col_length = d
            .first()
            .unwrap()
            .len();

        let mut row_idx = 0;

        let mut connected_corners = HashMap::new();

        connected_corners.insert('┌', '┐');
        connected_corners.insert('└', '┘');

        while row_idx < row_length {
            let mut col_idx = 0;

            while col_idx < col_length {
                print!("{}", d[row_idx][col_idx]);

                if d[row_idx][col_idx] == '.' {
                    let mut passed = 0;

                    let mut c_idx = col_idx;

                    let mut corner = None;
                    let mut corner_char = None;

                    while c_idx < col_length {
                        if d[row_idx][c_idx] != '-' && d[row_idx][c_idx] != '.' {
                            if d[row_idx][c_idx] != '|' {
                                if d[row_idx][c_idx] == '*' {
                                    // if corner.is_some() {
                                    //     c_idx += 1;
                                    //     continue;
                                    // }
                                } else if let Some(crn) = corner {
                                    if d[row_idx][c_idx] == crn {
                                        corner = None;
                                    } else if Some(d[row_idx][c_idx]) != corner_char {
                                        c_idx += 1;
                                        continue;
                                    }
                                } else {
                                    if let Some(crn) = connected_corners.get(&d[row_idx][c_idx]) {
                                        corner_char = Some(d[row_idx][c_idx]);
                                        corner = Some(*crn);
                                    }
                                }
                            }

                            passed += 1;
                        }

                        c_idx += 1;
                    }

                    if passed % 2 != 0 {
                        enclose_tile_count += 1;
                    }
                }

                col_idx += 1;
            }

            println!("=== {enclose_tile_count} ===");

            row_idx += 1;
        }

        enclose_tile_count
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
        let sample_test_case = "...........\n\
        .S-------7.\n\
        .|F-----7|.\n\
        .||.....||.\n\
        .||.....||.\n\
        .|L-7.F-J|.\n\
        .|..|.|..|.\n\
        .L--J.L--J.\n\
        ...........";

        assert_eq!(Puzzle::solve(sample_test_case), 4);

        let sample_test_case = ".F----7F7F7F7F-7....\n\
        .|F--7||||||||FJ....\n\
        .||.FJ||||||||L7....\n\
        FJL7L7LJLJ||LJ.L-7..\n\
        L--J.L7...LJS7F-7L7.\n\
        ....F-J..F7FJ|L7L7L7\n\
        ....L7.F7||L7|.L7L7|\n\
        .....|FJLJ|FJ|F7|.LJ\n\
        ....FJL-7.||.||||...\n\
        ....L---J.LJ.LJLJ...";

        assert_eq!(Puzzle::solve(sample_test_case), 8);

        let sample_test_case = "FF7FSF7F7F7F7F7F---7\n\
        L|LJ||||||||||||F--J\n\
        FL-7LJLJ||||||LJL-77\n\
        F--JF--7||LJLJ7F7FJ-\n\
        L---JF-JLJ.||-FJLJJ7\n\
        |F|F-JF---7F7-L7L|7|\n\
        |FFJF7L7F-JF7|JL---7\n\
        7-L-JL7||F7|L7F-7F7|\n\
        L.L7LFJ|||||FJL7||LJ\n\
        L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(Puzzle::solve(sample_test_case), 10);

        Ok(())
    }
}
