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

            let games_play = l.split(':').collect::<Vec<&str>>();

            let game_id = games_play[0];

            let mut is_increment = true;

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
                        is_increment = false;
                        break;
                    }
                }
            }

            if is_increment {
                answers += i32::from_str_radix(game_id
                    .matches(char::is_numeric)
                    .fold("".to_string(), |acc, c: &str| format!("{acc}{c}")).as_str(), 10).unwrap();
            }
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

        assert_eq!(Puzzle::solve(sample_test_case), 8);

        Ok(())
    }
}
