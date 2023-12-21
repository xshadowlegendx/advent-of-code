use std::collections::HashMap;


pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> usize {
        let moves = HashMap::from([
            (("U", "U"), (-1, 0)),
            (("U", "L"), (0, -1)),
            (("U", "R"), (0, 1)),

            (("D", "D"), (1, 0)),
            (("D", "L"), (0, -1)),
            (("D", "R"), (0, 1)),

            (("L", "L"), (0, -1)),
            (("L", "D"), (1, 0)),
            (("L", "U"), (-1, 0)),

            (("R", "R"), (0, 1)),
            (("R", "D"), (1, 0)),
            (("R", "U"), (-1, 0)),
        ]);

        let mut previous_move = None;

        let mut current_position = (0, 0);

        let mut dugged = vec![];

        let mut dugged_position = HashMap::<(i32, i32), ()>::new();

        let mut max_row = 0;
        let mut max_col = 0;

        let mut min_row = 0;
        let mut min_col = 0;

        let mut max_size = 0;

        for l in input.split('\n') {
            let mut dir = None;
            let mut range = None;
            let mut color = None;

            for plan in l.split(' ') {
                if dir.is_none() {
                    dir = Some(plan);
                } else if range.is_none() {
                    range = Some(i32::from_str_radix(plan, 10).unwrap());
                } else if color.is_none() {
                    color = Some(plan);
                }
            }

            let dir = dir.unwrap();
            let range = range.unwrap();
            let _color = color.unwrap();

            if previous_move.is_none() {
                previous_move = Some(dir);
            }

            let next_move = moves
                .get(&(previous_move.unwrap(), dir))
                .unwrap();

            previous_move = Some(dir);

            let dug_til_position = (current_position.0 + next_move.0 * range, current_position.1 + next_move.1 * range);

            dugged.push((current_position, dug_til_position, next_move));

            current_position = dug_til_position;

            if current_position.0 > max_row {
                max_row = current_position.0;
            }

            if current_position.1 > max_row {
                max_col = current_position.1;
            }

            if current_position.0 < min_row {
                min_row = current_position.0;
            }

            if current_position.1 < min_col {
                min_col = current_position.1;
            }
        }

        max_col += 2;
        max_row += 2;

        min_row -= 2;
        min_col -= 2;

        if max_col.abs() > max_size {
            max_size = max_col.abs();
        }

        if max_row.abs() > max_size {
            max_size = max_row.abs();
        }

        if min_row.abs() > max_size {
            max_size = min_row.abs();
        }
        
        if min_col.abs() > max_size {
            max_size = min_col.abs();
        }

        max_size *= 4;

        let mut map = vec![vec!['.'; max_size as usize];max_size as usize];

        let mut edges = vec![];

        let mut edges_map = HashMap::new();

        for d in &dugged {
            let mut r_idx = d.0.0;
            let mut c_idx = d.0.1;

            while (r_idx, c_idx) != d.1 {
                let map_row = ((max_size / 2) + r_idx) as usize;
                let map_col = ((max_size / 2) + c_idx) as usize;

                edges.push((map_row, map_col));

                edges_map.insert((map_row, map_col), ());

                map[map_row][map_col] = '#';

                dugged_position.insert((r_idx, c_idx), ());

                r_idx += d.2.0;
                c_idx += d.2.1;
            }
        }

        let mut dugged_deep = HashMap::new();

        // for m in &map {
        //     println!("{}", String::from_utf8(m.iter().map(|f| *f as u8).collect()).unwrap());
        // }

        for edge in edges {
            let mut col_idx = edge.1;

            while col_idx < max_size as usize {
                if dugged_deep.contains_key(&(edge.0, col_idx)) {
                    col_idx += 1;
                    continue;
                }

                if map[edge.0][col_idx] == '.' {
                    let mut idx = 0;
                    let mut passes = 0;

                    let mut upper_edge_count = 0;
                    let mut bottom_edge_count = 0;

                    while col_idx + idx < max_size as usize {
                        if edges_map.contains_key(&(edge.0, col_idx + idx)) {
                            if edges_map.contains_key(&(edge.0 + 1, col_idx + idx)) {
                                upper_edge_count += 1;
                            }

                            if edges_map.contains_key(&(edge.0 - 1, col_idx + idx)) {
                                bottom_edge_count += 1;
                            }

                            if upper_edge_count == 1 && bottom_edge_count == 1 {
                                passes += 1;
                                upper_edge_count = 0;
                                bottom_edge_count = 0;
                            } else if upper_edge_count == 2 || bottom_edge_count == 2 {
                                passes += 2;
                                upper_edge_count = 0;
                                bottom_edge_count = 0;
                            }
                        }

                        idx += 1;
                    }

                    if passes % 2 != 0 {
                        map[edge.0][col_idx] = '#';
                        dugged_deep.insert((edge.0, col_idx), ());

                        // for m in &map {
                        //     println!("{}", String::from_utf8(m.iter().map(|f| *f as u8).collect()).unwrap());
                        // }
                    }
                }

                col_idx += 1;
            }
        }

        // for m in &map {
        //     println!("{}", String::from_utf8(m.iter().map(|f| *f as u8).collect()).unwrap());
        // }

        dugged_position.len() + dugged_deep.len()
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
        let sample_test_case = "R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)";

        assert_eq!(Puzzle::solve(sample_test_case), 62);

        Ok(())
    }
}
