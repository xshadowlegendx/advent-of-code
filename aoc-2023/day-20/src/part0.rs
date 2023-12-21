use std::collections::{VecDeque, HashMap};


pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str, steps_to_take: i32) -> usize {
        let mut map = vec![];

        let mut start_pos = (0, 0);

        for l in input.split('\n') {
            map.push(l.to_string());

            if let Some(y) = l.find('S') {
                start_pos.0 = map.len() - 1;
                start_pos.1 = y;
            }
        }

        let row_length = map.len();
        let col_length = map
            .first()
            .unwrap()
            .len();

        let mut current_steps = HashMap::new();

        let mut to_spreads = VecDeque::new();

        to_spreads.push_back(start_pos);

        let mut steps_to_take = steps_to_take;

        while steps_to_take > 0 {
            let mut steps = HashMap::new();

            let mut new_spreads = VecDeque::new();

            while let Some(pos) = to_spreads.pop_front() {
                for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next_loc = (
                        pos.0 as i32 + dir.0,
                        pos.1 as i32 + dir.1
                    );

                    if next_loc.0 >= row_length as i32 || next_loc.1 >= col_length as i32 || next_loc.0 < 0 || next_loc.1 < 0 {
                        continue;
                    }

                    let next_loc = (
                        next_loc.0 as usize,
                        next_loc.1 as usize
                    );

                    if (map[next_loc.0].as_bytes()[next_loc.1] as char) == '#' {
                        continue;
                    }

                    if steps.contains_key(&next_loc) {
                        continue;
                    }

                    steps.insert(next_loc, ());

                    new_spreads.push_back(next_loc);
                }
            }

            // debug
            // let mut r_idx = 0;
            // while r_idx < row_length {
            //     let mut c_idx = 0;

            //     while c_idx < col_length {
            //         if steps.contains_key(&(r_idx, c_idx)) {
            //             print!("O");
            //         } else {
            //             print!("{}", map[r_idx].as_bytes()[c_idx] as char);
            //         }

            //         c_idx += 1;
            //     }

            //     println!("");

            //     r_idx += 1;
            // }
            // println!("====== iter: {steps_to_take} =====");
            // debug

            to_spreads = new_spreads;

            current_steps = steps;

            steps_to_take -= 1;
        }
        
        current_steps.len()
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
        .....###.#.\n\
        .###.##..#.\n\
        ..#.#...#..\n\
        ....#.#....\n\
        .##..S####.\n\
        .##..#...#.\n\
        .......##..\n\
        .##.#.####.\n\
        .##..##.##.\n\
        ...........";

        assert_eq!(Puzzle::solve(sample_test_case, 6), 16);

        Ok(())
    }
}
