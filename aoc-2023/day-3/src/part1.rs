
use std::collections::{HashSet, HashMap, VecDeque};

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut winings_card = HashMap::new();

        let cards = input
            .split('\n')
            .collect::<Vec<&str>>();

        let mut cards_to_process = VecDeque::from_iter(1..cards.len() + 1);

        let mut cards_map = HashMap::new();

        while let Some(card_idx) = cards_to_process.pop_front() {
            let line = cards[card_idx - 1 as usize];

            if !winings_card.contains_key(&card_idx) {
                winings_card.insert(card_idx, 0);
            }

            let card = cards_map.get(&card_idx);

            if card.is_none() {
                let mut winners = HashSet::new();

                let parts = line
                    .split('|')
                    .collect::<Vec<&str>>();

                let winnings = parts[0]
                    .split(':')
                    .collect::<Vec<&str>>();

                let card_id = i32::from_str_radix(&winnings[0]
                        .replace(char::is_whitespace, "")
                    .replace(char::is_alphabetic, ""), 10).unwrap();

                let winnings = winnings[1]
                    .split(char::is_whitespace)
                    .filter(|n| !n.is_empty())
                    .map(|n| {
                        i32::from_str_radix(n, 10).unwrap()
                    })
                    .collect::<Vec<i32>>();

                let my_numbers = parts[1]
                    .split(char::is_whitespace)
                    .filter(|n| !n.is_empty())
                    .map(|n| {
                        i32::from_str_radix(n, 10).unwrap()
                    })
                    .collect::<Vec<i32>>();

                for num in winnings.iter() {
                    for n in my_numbers.iter() {
                        if num == n {
                            winners.insert(*num);
                        }
                    }
                }

                cards_map.insert(card_idx, (card_id, winners));
            }

            let (card_id, winners) = cards_map.get(&card_idx).unwrap();

            if winners.len() > 0 {
                for idx in 0..((winners.len() + 1) as i32) {
                    let idx = idx + card_id;

                    if idx != card_idx as i32 {
                        cards_to_process.push_back(idx as usize);
                    }
                }
            }

            *winings_card.get_mut(&card_idx).unwrap() += 1;
        }

        winings_card
            .values()
            .fold(0, |acc, val| {
                if *val == 0 {
                    acc + 1
                } else {
                    acc + val
                }
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
        let sample_test_case = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(Puzzle::solve(sample_test_case), 30);

        Ok(())
    }
}
