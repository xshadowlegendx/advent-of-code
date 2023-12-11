
use std::collections::HashMap;

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str, expand_amount: usize) -> usize {
        let expand_amount = expand_amount - 1;

        let mut maps = vec![];

        let mut expanded_row = vec![];
        let mut expanded_col = vec![];

        for l in input.split('\n') {
            maps.push(vec![]);

            let mut is_galaxy_found = false;

            for ch in l.chars() {
                if ch == '#' {
                    is_galaxy_found = true;

                    maps
                        .last_mut()
                        .unwrap()
                        .push('*');
                } else {
                    maps
                        .last_mut()
                        .unwrap()
                        .push('.');
                }
            }

            if !is_galaxy_found {
                // maps.push(vec!['.'; len]);
                expanded_row.push(maps.len() - 1);
            }
        }

        let row_length = maps.len();
        let col_length = maps
            .first()
            .unwrap()
            .len();

        let mut col_idx = 0;

        let mut mmaps = vec![vec![]; row_length];

        while col_idx < col_length {
            let mut row_idx = 0;
            let mut is_found_galaxy = false;

            while row_idx < row_length {
                if maps[row_idx][col_idx] != '.' {
                    is_found_galaxy = true;
                }

                mmaps[row_idx].push(maps[row_idx][col_idx]);

                row_idx += 1;
            }

            if !is_found_galaxy {
                expanded_col.push(col_idx);

                // while row_idx < row_length {
                //     mmaps[row_idx].push('.');

                //     row_idx += 1;
                // }
            }

            col_idx += 1;
        }

        let mut row_idx = 0;

        let row_length = mmaps.len();
        let col_length = mmaps
            .first()
            .unwrap()
            .len();

        let mut galaxies_loc = HashMap::new();
        let mut galaxies_id_loc = HashMap::new();

        while row_idx < row_length {
            let mut col_idx = 0;

            while col_idx < col_length {
                if mmaps[row_idx][col_idx] == '*' {
                    galaxies_id_loc.insert(
                        galaxies_loc.len() + 1,
                        (row_idx, col_idx)
                    );
                    galaxies_loc.insert(
                        (row_idx, col_idx),
                        galaxies_loc.len() + 1,
                    );
                }

                col_idx += 1;
            }

            row_idx += 1;
        }

        let mut galaxy_pairs_shortest_path = HashMap::new();

        let mut first_of_pair = 1;

        while first_of_pair < galaxies_id_loc.len() + 1 {
            let mut second_of_pair = first_of_pair + 1;

            while second_of_pair < galaxies_id_loc.len() + 1 {
                let first_pair_loc = galaxies_id_loc
                    .get(&first_of_pair)
                    .unwrap();

                let second_pair_loc = galaxies_id_loc
                    .get(&second_of_pair)
                    .unwrap();

                let mut within_expanded_row = 0;

                for er in &expanded_row {
                    if first_pair_loc.0 < *er && *er < second_pair_loc.0 {
                        within_expanded_row += expand_amount;
                    }

                    if  second_pair_loc.0 < *er && *er < first_pair_loc.0 {
                        within_expanded_row += expand_amount;
                    }
                }

                let row_diff =
                    if first_pair_loc.0 > second_pair_loc.0 {
                        (first_pair_loc.0 + within_expanded_row).abs_diff(second_pair_loc.0)
                    } else {
                        (first_pair_loc.0).abs_diff(second_pair_loc.0 + within_expanded_row)
                    };

                let mut within_expanded_col = 0;

                for ec in &expanded_col {
                    if first_pair_loc.1 < *ec && *ec < second_pair_loc.1 {
                        within_expanded_col += expand_amount;
                    }

                    if second_pair_loc.1 < *ec && *ec < first_pair_loc.1 {
                        within_expanded_col += expand_amount;
                    }
                }

                let col_diff =
                    if first_pair_loc.1 > second_pair_loc.1 {
                        (first_pair_loc.1 + within_expanded_col).abs_diff(second_pair_loc.1)
                    } else {
                        (first_pair_loc.1).abs_diff(second_pair_loc.1 + within_expanded_col)
                    };

                galaxy_pairs_shortest_path.insert((first_of_pair, second_of_pair), row_diff + col_diff);

                second_of_pair += 1;
            }

            first_of_pair += 1;
        }

        galaxy_pairs_shortest_path
            .iter()
            .fold(0, |acc, (_, shortest_path)| {
                acc + shortest_path
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
        let sample_test_case = "...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....";

        // .....1..........
        // ...........2....
        // 3...............
        // ................
        // ................
        // ................
        // ..........4.....
        // .5..............
        // ...............6
        // ................
        // ................
        // ................
        // ...........7....
        // 8.....9.........

        assert_eq!(Puzzle::solve(sample_test_case, 10), 1030);

        assert_eq!(Puzzle::solve(sample_test_case, 100), 8410);

        Ok(())
    }
}
