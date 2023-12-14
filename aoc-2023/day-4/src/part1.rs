
use std::collections::BTreeMap;

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> i64 {
        let mut idx = 0;

        let mut initials = vec![];

        let mut to_convert = vec![];

        let mut matches = BTreeMap::<i64, (i64, bool)>::new();

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
            for (conv_dest, conv_src, conv_range) in conv {
                let mut idx = 0;

                while idx < initials.len() {
                    let ranges = initials
                        .get_mut(idx)
                        .unwrap();

                    let mut new_ranges = vec![];

                    for (start, range) in ranges {
                        let min = conv_src;
                        let max = conv_src + conv_range;

                        let max_seed = *start + *range;

                        if min <= *start && max_seed <= max { // center
                            new_ranges.push(((*start - min) + conv_dest, *range));
                        } else if min >= *start && max <= max_seed { // cover all
                            new_ranges.push((*start, min - *start));

                            new_ranges.push((conv_dest + conv_range, max - max_seed));

                            new_ranges.push((conv_dest, conv_range));
                        } else if min >= *start && max >= max_seed && max_seed >= min { // head part
                            new_ranges.push((*start, min - *start));

                            new_ranges.push((conv_dest, max_seed - min));
                        } else if min <= *start && max <= max_seed && max >= *start { // tail part
                            new_ranges.push((max, max_seed - max));

                            new_ranges.push((conv_dest + *start - min, max - *start));
                        } else {
                            new_ranges.push((*start, *range));
                        }
                    }

                    *initials
                        .get_mut(idx)
                        .unwrap() = new_ranges;

                    idx += 1;
                }
            }
        }

        46
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
