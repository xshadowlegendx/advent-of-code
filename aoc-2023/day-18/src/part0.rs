
use std::collections::{HashMap, VecDeque};

pub struct Puzzle;

impl Puzzle {
    #[tracing::instrument]
    pub fn solve(input: &str) -> u32 {
        let mut workflows = HashMap::<&str, Vec<(String, char, u32, String)>>::new();

        let mut is_start_process_ratings = false;

        let mut answer = 0;

        for l in input.split('\n') {
            if l == "" {
                is_start_process_ratings = true;
                continue;
            }

            if is_start_process_ratings {
                let mut checks = HashMap::<String, u32>::new();

                for c in l.split(',') {
                    let (match_c, num) = c
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split_at(1);

                    checks.insert(match_c.to_string(), u32::from_str_radix(&num.replace("=", ""), 10).unwrap());
                }

                let mut workflow_comp = VecDeque::new();

                workflow_comp.push_back("in".to_string());

                while let Some(r) = workflow_comp.pop_front() {
                    if r == "A" {
                        answer += checks
                            .iter()
                            .fold(0, |acc, (_, num)| {
                                acc + num
                            });
                        break;
                    }

                    if r == "R" {
                        break;
                    }

                    let cons = workflows
                        .get(r.as_str())
                        .unwrap();

                    let mut last_match = None;
                    let mut next_match = None;

                    for c in cons {
                        if next_match.is_some() {
                            break;
                        }

                        if let Some(chk) = checks.get(&c.0) {
                            if c.1 == '>' {
                                if *chk > c.2 {
                                    next_match = Some(c.3.clone());
                                }
                            } else if c.1 == '<' {
                                if *chk < c.2 {
                                    next_match = Some(c.3.clone());
                                }
                            }
                        }

                        last_match = Some(c.3.clone());
                    }

                    if let Some(next) = next_match {
                        workflow_comp.push_back(next);
                    } else if let Some(next) = last_match {
                        workflow_comp.push_back(next);
                    }
                }

                continue;
            }

            let (rating, rest) = l.split_at(l.find('{').unwrap());

            workflows.insert(rating, vec![]);

            let workflow_rating = workflows
                .get_mut(rating)
                .unwrap();

            for l in rest.split(',') {
                if let Some(con) = l.find(':') {
                    let (con, next) = l.split_at(con);

                    let mut idx = 0;

                    let mut op = None;

                    for ch in con.chars() {
                        if op.is_some() {
                            break;
                        }

                        if ch == '>' || ch == '<' {
                            op = Some(ch);
                            break;
                        }

                        idx += 1;
                    }

                    let (part_match, num_comp) = con.split_at(idx);

                    workflow_rating.push((
                        part_match.replace("{", ""),
                        op.unwrap(),
                        u32::from_str_radix(num_comp.trim_start_matches('<').trim_start_matches('>'), 10).unwrap(),
                        next.replace(":", "")
                    ));
                } else {
                    workflow_rating.push(("".to_string(), '.', 0, l.replace("}", "")));
                }
            }
        }

        answer
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
        let sample_test_case = "px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\
        \n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(Puzzle::solve(sample_test_case), 19114);

        Ok(())
    }
}
