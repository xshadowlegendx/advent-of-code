use std::collections::{VecDeque, HashMap};


pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut instructions = VecDeque::new();

        let mut maps = HashMap::new();

        let mut positions = vec![];

        for l in input.split('\n') {
            if instructions.is_empty() {
                instructions = l.chars().collect();
                continue;
            }

            let mut map_id = None;

            for s in l.split('=') {
                if map_id.is_none() {
                    map_id = Some(s.trim());

                    if s.trim().ends_with('A') {
                        positions.push(s.trim());
                    }

                    maps.insert(s.trim(), HashMap::new());
                    continue;
                }

                let mut is_left = true;

                for next in s.split(',') {
                    if is_left {
                        let left = next
                            .replace('(', "")
                            .trim()
                            .to_string();

                        maps
                            .get_mut(map_id.unwrap())
                            .unwrap()
                            .insert('L', left);

                        is_left = false;
                        continue;
                    }

                    let right = next
                        .replace(')', "")
                        .trim()
                        .to_string();

                    maps
                        .get_mut(map_id.unwrap())
                        .unwrap()
                        .insert('R', right);
                }
            }
        }

        let mut steps = 0;

        let mut ghost_steps = vec![HashMap::new(); positions.len()];

        while let Some(direction) = instructions.pop_front() {
            steps += 1;
            instructions.push_back(direction);

            let mut pos_idx = 0;
            let mut goals_found = 0;

            while pos_idx < positions.len() {
                let position = positions.get_mut(pos_idx).unwrap();

                let next = maps.get(position).unwrap();

                let next_position = next.get(&direction).unwrap();

                if next_position.ends_with('Z') {
                    ghost_steps
                        .get_mut(pos_idx)
                        .unwrap()
                        .insert(steps, (position.to_string(), next_position.to_string()));

                    goals_found += 1;
                }

                *position = next_position.as_str();

                pos_idx += 1;
            }

            if goals_found == positions.len() {
                break;
            }
        }

        steps
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
        let sample_test_case = "LR\n
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)";

        assert_eq!(Puzzle::solve(sample_test_case), 6);

        let sample_test_case = "LRR\n\
        11A = (11Z, 11B)\n\
        11B = (11A, 11C)\n\
        11C = (11B, 11Z)\n\
        11Z = (11C, 11A)\n\
        22A = (22B, 22Z)\n\
        22B = (22Z, 22A)\n\
        22Z = (22A, 22B)";

        assert_eq!(Puzzle::solve(sample_test_case), 21);

        Ok(())
    }
}
