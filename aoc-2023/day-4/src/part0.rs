use std::collections::{BTreeSet, BTreeMap};

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i64 {
        let mut idx = 0;

        let mut checks = BTreeSet::new();

        let mut matches = BTreeMap::new();

        for l in input.split('\n') {
            if idx == 0 {
                for seed in l.replace("seeds: ", "").split(' ') {
                    let seed = i64::from_str_radix(seed, 10).unwrap();

                    matches.insert(seed, seed);
                }
            }

            if idx > 0 && !l.is_empty() {
                if l.as_bytes()[0] > 96 {
                    checks.clear();

                    while let Some((_, val)) = matches.pop_first() {
                        checks.insert(val);
                    }

                    for c in &checks {
                        matches.insert(*c, *c);
                    }
                } else {
                    let mut nums = vec![];

                    let mut iter_idx = 0;

                    for num in l.split(' ') {
                        let num = i64::from_str_radix(num, 10)
                            .unwrap();

                        if iter_idx == 2 {
                            let min = nums[1];
                            let max = nums[1] + num;

                            for c in &checks {
                                if *c >= min && *c <= max {
                                    *matches.get_mut(c).unwrap() = *c - min + nums[0];
                                }
                            }

                            continue;
                        }

                        nums.push(num);

                        iter_idx += 1;
                    }
                }
            }

            idx += 1;
        }

        matches
            .pop_first()
            .unwrap()
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
        let sample_test_case = "seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4";

        assert_eq!(Puzzle::solve(sample_test_case), 35);

        Ok(())
    }
}
