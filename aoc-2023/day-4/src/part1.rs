use std::collections::HashMap;


pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i64 {
        let mut idx = 0;

        let mut initials = vec![];

        let mut to_convert = vec![];

        for l in input.split('\n') {
            if idx == 0 {
                let mut first = None;

                for seed in l.replace("seeds: ", "").split(' ') {
                    if seed.is_empty() {
                        continue;
                    }

                    let seed = i64::from_str_radix(seed, 10).unwrap();

                    if let Some(start) = first {
                        initials.push(vec![(start, seed)]);

                        first = None;
                    } else {
                        first = Some(seed);
                    }
                }
            }

            if idx > 0 && !l.is_empty() {
                if l.as_bytes()[0] > 96 {
                    to_convert.push(vec![]);
                } else {
                    let mut nums = vec![];

                    for num in l.split(' ') {
                        let num = i64::from_str_radix(num, 10).unwrap();

                        nums.push(num);
                    }

                    to_convert
                        .last_mut()
                        .unwrap()
                        .push((nums[0], nums[1], nums[2]));
                }
            }

            idx += 1;
        }

        for conv in to_convert {
            let mut is_changed = HashMap::new();

            for (conv_dest, conv_src, conv_range) in conv {
                let mut idx = 0;

                if conv_dest == 0 {
                    println!("now zero");
                }

                while idx < initials.len() {
                    let ranges = initials
                        .get_mut(idx)
                        .unwrap();

                    let mut new_ranges = vec![];

                    for (start, range) in ranges.iter() {
                        let min = conv_src;
                        let max = conv_src + conv_range;

                        let max_seed = *start + *range;

                        if min <= *start && max_seed <= max { // center
                            new_ranges.push(((*start - min) + conv_dest, *range));
                        } else if min >= *start && max <= max_seed { // cover all
                            new_ranges.push((*start, min - *start - 1));

                            new_ranges.push((max + 1, max_seed - max - 1));

                            new_ranges.push((conv_dest, conv_range));
                        } else if min >= *start && max >= max_seed && max_seed >= min { // head part
                            new_ranges.push((*start, min - *start - 1));

                            if max_seed - min == 0 {
                                new_ranges.push((conv_dest, 1));
                            } else {
                                new_ranges.push((conv_dest, max_seed - min));
                            }
                        } else if min <= *start && max <= max_seed && max >= *start { // tail part
                            new_ranges.push((max + 1, max_seed - max - 1));

                            if max - *start == 0 {
                                new_ranges.push((conv_dest, 1));
                            } else {
                                new_ranges.push((conv_dest + *start - min, max_seed - max));
                            }
                        } else {
                            new_ranges.push((*start, *range));
                        }
                    }

                    if new_ranges != *ranges {
                        if !is_changed.contains_key(&idx) {
                            is_changed.insert(idx, ());

                            *initials
                                .get_mut(idx)
                                .unwrap() = new_ranges;
                        } else {
                            let lowest_new_init = new_ranges
                                .iter()
                                .fold(i64::MAX, |lowest, (v, _)| {
                                    if lowest > *v {
                                        *v
                                    } else {
                                        lowest
                                    }
                                });

                            let lowest_init = ranges
                                .iter()
                                .fold(i64::MAX, |lowest, (v, _)| {
                                    if lowest > *v {
                                        *v
                                    } else {
                                        lowest
                                    }
                                });

                            if lowest_init > lowest_new_init {
                                let _ = lowest_init + lowest_new_init;
                                // *initials
                                //     .get_mut(idx)
                                //     .unwrap() = new_ranges;
                            }
                        }
                    }

                    idx += 1;
                }
            }
        }

        let mut lowest = i64::MAX;

        for init in &initials {
            for (l, _) in init {
                if lowest > *l {
                    lowest = *l;
                }
            }
        }

        lowest
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

        assert_eq!(Puzzle::solve(sample_test_case), 46);

        Ok(())
    }
}
