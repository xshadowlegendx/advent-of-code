
use std::collections::HashMap;

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut answers = 0;

        for l in input.split('\n') {
            let mut colors = HashMap::new();

            colors.insert("red", 12);
            colors.insert("green", 13);
            colors.insert("blue", 14);

            let mut colors_at_most = HashMap::new();

            let mut colors_at_least = HashMap::new();

            let games_play = l.split(':').collect::<Vec<&str>>();

            for game in games_play[1].split(';') {
                for game in game.split(',') {
                    let g = game
                        .trim()
                        .split(' ')
                        .collect::<Vec<&str>>();

                    let color = colors.get(g[1]).unwrap();

                    let count = i32::from_str_radix(g[0], 10)
                        .unwrap();

                    if color - count < 0 {
                        if let Some(at_least_count) = colors_at_least.get(g[1]) {
                            if *at_least_count > count {
                                continue;
                            }
                        }

                        colors_at_least.insert(g[1], count);
                    } else {
                        if let Some(at_most_count) = colors_at_most.get(g[1]) {
                            if *at_most_count > count {
                                continue
                            }
                        }

                        colors_at_most.insert(g[1], count);
                    }
                }
            }

            let mut power_set = 1;

            for color in ["red", "green", "blue"] {
                if let Some(val) = colors_at_least.get(color) {
                    power_set *= val;
                } else if let Some(val) = colors_at_most.get(color) {
                    power_set *= val;
                }
            }

            answers += power_set;
        }

        answers
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
        let sample_test_case = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(Puzzle::solve(sample_test_case), 2286);

        Ok(())
    }
}
