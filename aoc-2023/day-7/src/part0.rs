use std::collections::{HashMap, VecDeque};


pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut instructions = VecDeque::new();

        let mut maps = HashMap::new();

        for l in input.split('\n') {
            if instructions.is_empty() {
                instructions = l.chars().collect();
                continue;
            }

            let mut map_id = None;

            for s in l.split('=') {
                if map_id.is_none() {
                    map_id = Some(s.trim());

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

        let mut position = "AAA";
        let mut steps = 0;

        while let Some(direction) = instructions.pop_front() {
            steps += 1;
            instructions.push_back(direction);

            let next = maps.get(position).unwrap();

            let next_position = next.get(&direction).unwrap();

            if *next_position == "ZZZ" {
                break;
            }

            position = next_position;
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
        let sample_test_case = "LLR\n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(Puzzle::solve(sample_test_case), 6);

        Ok(())
    }
}
