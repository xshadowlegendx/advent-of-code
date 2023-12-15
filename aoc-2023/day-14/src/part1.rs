
use std::collections::HashMap;


pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> usize {
        let mut label_loc = HashMap::new();
        let mut boxes = vec![vec![];256];

        for s in input.split(',') {
            let mut last_ch = '0';
            let mut ops = None;
            let mut label = String::new();
            let mut label_idx = 0;

            for ch in s.chars() {
                last_ch = ch;

                if ops.is_none() {
                    if ch == '-' || ch == '=' {
                        ops = Some(ch);
                    } else {
                        label.push(ch);
                        label_idx += (ch as u8) as usize;
                        label_idx *= 17;
                        label_idx %= 256;
                    }
                }
            }

            let ops = ops.unwrap();

            if ops == '-' {
                if let Some(box_idx) = label_loc.get(&label) {
                    boxes[*box_idx as usize] = boxes
                        .get(*box_idx as usize)
                        .unwrap()
                        .iter()
                        .filter(|(l, _)| l != &label)
                        .cloned()
                        .collect();
                    label_loc.remove(&label);
                }
            } else {
                if label_loc.contains_key(&label) {
                    for v in boxes.get_mut(label_idx).unwrap().iter_mut() {
                        if v.0 == label {
                            v.1 = last_ch.to_digit(10).unwrap();
                        }
                    }
                } else {
                    boxes
                        .get_mut(label_idx)
                        .unwrap()
                        .push((label.clone(), last_ch.to_digit(10).unwrap()));
                    label_loc.insert(label.to_owned(), label_idx);
                }
            }
        }

        let mut ans = 0;

        let mut box_idx = 0;

        while box_idx < boxes.len() {
            let mut val_idx = 0;

            while val_idx < boxes[box_idx].len() {
                ans += (box_idx + 1) * (val_idx + 1) * boxes[box_idx][val_idx].1 as usize;

                val_idx += 1;
            }

            box_idx += 1;
        }

        ans
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
        let sample_test_case = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(Puzzle::solve(sample_test_case), 145);

        Ok(())
    }
}
