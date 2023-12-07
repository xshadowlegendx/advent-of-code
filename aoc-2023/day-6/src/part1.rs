
use std::collections::{HashMap, BTreeMap};

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i32 {
        let mut strengthes = HashMap::new();
        strengthes.insert('A', 0);
        strengthes.insert('K', 1);
        strengthes.insert('Q', 2);
        strengthes.insert('T', 3);
        strengthes.insert('9', 4);
        strengthes.insert('8', 5);
        strengthes.insert('7', 6);
        strengthes.insert('6', 7);
        strengthes.insert('5', 8);
        strengthes.insert('4', 9);
        strengthes.insert('3', 10);
        strengthes.insert('2', 11);
        strengthes.insert('J', 12);

        let mut hands = BTreeMap::new();

        input
            .split('\n')
            .for_each(|l| {
                let mut hand = String::new();
                let mut bid = String::new();

                let mut is_start_collect_bid = false;

                let mut kinds_classifier = HashMap::new();

                let mut last = None;

                for c in l.chars() {
                    if c == ' ' {
                        is_start_collect_bid = true;
                        continue;
                    }

                    if is_start_collect_bid {
                        bid.push(c);
                        continue;
                    }

                    if !kinds_classifier.contains_key(&c) {
                        kinds_classifier.insert(c, 0);
                    }

                    hand.push(c);

                    *kinds_classifier.get_mut(&c).unwrap() += 1;

                    last = Some((c, *kinds_classifier.get(&c).unwrap()));
                }

                let j = kinds_classifier.get(&'J');

                let (_, kind_jed) = match kinds_classifier.len() {
                    1 => (6, 6),
                    2 => {
                        let mut val = 5;

                        if let Some((_, count)) = &last {
                            if *count == 2 || *count == 3 {
                                val = 4;
                            }
                        }

                        if j.is_some() {
                            (val, 6)
                        } else {
                            (val, val)
                        }
                    },
                    3 => {
                        let mut is_three = false;

                        for (_, count) in &kinds_classifier {
                            if *count == 3 {
                                is_three = true;
                                break;
                            }
                        }

                        if is_three {
                            if j.is_some() {
                                (3, 5)
                            } else {
                                (3, 3)
                            }
                        } else {
                            if let Some(j_count) = j {
                                if *j_count == 2 {
                                    (2, 5)
                                } else if *j_count == 1 {
                                    (2, 4)
                                } else {
                                    (2, 3)
                                }
                            } else {
                                (2, 2)
                            }
                        }
                    },
                    4 => {
                        if j.is_some() {
                            (1, 3)
                        } else {
                            (1, 1)
                        }
                    },
                    _ => {
                        if j.is_some() {
                            (0, 1)
                        } else {
                            (0, 0)
                        }
                    },
                };

                if !hands.contains_key(&kind_jed) {
                    hands.insert(kind_jed, vec![]);
                }

                let bid = i32::from_str_radix(bid.as_str(), 10).unwrap();

                hands
                    .get_mut(&kind_jed)
                    .unwrap()
                    .push((hand.to_owned(), bid));
            });

        for (_, ranks) in hands.iter_mut() {
            ranks.sort_by(|(charsx, _), (charsy, _)| {
                let mut idx = 0;
                
                while idx < 5 {
                    let strex = strengthes.get(&charsx.chars().nth(idx).unwrap()).unwrap();
                    let strey = strengthes.get(&charsy.chars().nth(idx).unwrap()).unwrap();

                    if strex != strey {
                        return strey.cmp(strex);
                    }

                    idx += 1;
                }

                std::cmp::Ordering::Equal
            });
        }

        for (kind, hs) in &hands {
            println!("{:?}", hs);
            println!("==== {kind} ====");
        }

        hands
            .iter()
            .fold((1, 0), |(outer_idx, outer_sum), (_, hs)| {
                let (inner_idx, inner_sum) = 
                    hs.iter().fold((outer_idx, 0), |(inner_idx, inner_sum), (_, bid)| {
                        (inner_idx + 1,  bid * inner_idx + inner_sum)
                    });

                (inner_idx, inner_sum + outer_sum)
            })
            .1
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
        let sample_test_case = "32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483";

        assert_eq!(Puzzle::solve(sample_test_case), 5905);

        Ok(())
    }
}
