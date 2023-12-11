
use std::collections::HashMap;

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> usize {
        let mut maps = vec![];

        for l in input.split('\n') {
            maps.push(vec![]);

            let len = l.len();

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
                maps.push(vec!['.'; len]);
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
                let mut row_idx = 0;
                while row_idx < row_length {
                    mmaps[row_idx].push('.');

                    row_idx += 1;
                }
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

                // print!("{}", mmaps[row_idx][col_idx]);

                col_idx += 1;
            }

            // println!("");

            row_idx += 1;
        }

        // println!("{:?}", galaxies_loc);
        // println!("{:?}", galaxies_id_loc);

        let mut galaxy_pairs_shortest_path = HashMap::new();

        let mut first_of_pair = 1;

        while first_of_pair < galaxies_id_loc.len() + 1 {
            let mut second_of_pair = first_of_pair + 1;

            while second_of_pair < galaxies_id_loc.len() + 1 {
                // println!("{first_of_pair}");
                // println!("{second_of_pair}");
                // println!("=====");

                let first_pair_loc = galaxies_id_loc
                    .get(&first_of_pair)
                    .unwrap();

                let second_pair_loc = galaxies_id_loc
                    .get(&second_of_pair)
                    .unwrap();

                let row_diff = first_pair_loc.0.abs_diff(second_pair_loc.0);

                let col_diff = first_pair_loc.1.abs_diff(second_pair_loc.1);

                galaxy_pairs_shortest_path.insert((first_of_pair, second_of_pair), row_diff + col_diff);

                second_of_pair += 1;
            }

            first_of_pair += 1;
        }

        // println!("{:?}", galaxy_pairs_shortest_path);

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

        assert_eq!(Puzzle::solve(sample_test_case), 374);

        Ok(())
    }
}
