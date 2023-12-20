use itertools::Itertools;
use std::{collections::HashMap, ops::Range};

static INPUT: &'static str = include_str!("./example");

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Debug)]
enum Rule {
    Condition(Condition, String),
    Default(String),
}

#[derive(Debug, Clone)]
struct Condition {
    attr: char,
    cmp: Cmp,
    val: i32,
}

#[derive(Debug, Clone)]
enum Cmp {
    Lt,
    Gt,
}

fn parse() -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut split = INPUT.split("\n\n");

    let workflows: HashMap<String, Workflow> = split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let name = l.split("{").nth(0).unwrap().to_string();

            let rules = l
                .split("{")
                .nth(1)
                .unwrap()
                .replace("}", "")
                .split(",")
                .map(|r| {
                    if r.contains(":") {
                        let mut split = r.split(":");

                        let c = split.next().unwrap();

                        let attr = c.chars().nth(0).unwrap();

                        let cmp = match c.chars().nth(1).unwrap() {
                            '<' => Cmp::Lt,
                            '>' => Cmp::Gt,
                            o => panic!("{o}"),
                        };

                        let val = c
                            .chars()
                            .skip(2)
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap();

                        Rule::Condition(
                            Condition { attr, cmp, val },
                            split.next().unwrap().to_string(),
                        )
                    } else {
                        Rule::Default(r.to_string())
                    }
                })
                .collect();

            (name, Workflow { rules })
        })
        .collect();

    let parts = split
        .next()
        .unwrap()
        .lines()
        .map(|p| {
            let binding = p.replace("}", "").replace("{", "");
            let mut split = binding.split(",");

            let x = split
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let m = split
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let a = split
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let s = split
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            Part { x, m, a, s }
        })
        .collect::<Vec<Part>>();

    (workflows, parts)
}

fn run_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = "in".to_string();

    loop {
        if let Some(wf) = workflows.get(&current) {
            for rule in &wf.rules {
                match rule {
                    Rule::Condition(Condition { attr, cmp, val }, next) => {
                        let attr_val = match attr {
                            'x' => part.x,
                            'm' => part.m,
                            'a' => part.a,
                            's' => part.s,
                            o => panic!("{o}"),
                        };

                        let res = match cmp {
                            Cmp::Lt => attr_val < *val,
                            Cmp::Gt => attr_val > *val,
                        };

                        if res {
                            current = next.to_string();
                            break;
                        }
                    }
                    Rule::Default(next) => {
                        current = next.to_string();
                        continue;
                    }
                }
            }
        } else {
            return current == 'A'.to_string();
        }
    }
}

// #[derive(Debug)]
// struct AbstractPart {
//     x: Vec<Condition>,
//     m: Vec<Condition>,
//     a: Vec<Condition>,
//     s: Vec<Condition>,
// }

// fn run_abstract_part(part: &AbstractPart, workflows: &HashMap<String, Workflow>) {
//     let mut current = "in".to_string();

//     if let Some(wf) = workflows.get(&current) {
//         for rule in &wf.rules {
//             match rule {
//                 Rule::Condition(Condition { attr, cmp, val }, next) => {
//                     let attr_val = match attr {
//                         'x' => part.x,
//                         'm' => part.m,
//                         'a' => part.a,
//                         's' => part.s,
//                         o => panic!("{o}"),
//                     };

//                     let res = match cmp {
//                         Cmp::Lt => attr_val < *val,
//                         Cmp::Gt => attr_val > *val,
//                     };

//                     if res {
//                         current = next.to_string();
//                         break;
//                     } else {
//                     }
//                 }
//                 Rule::Default(next) => {
//                     current = next.to_string();
//                     continue;
//                 }
//             }
//         }
//     } else {
//         return current == 'A'.to_string();
//     }
// }

pub fn one() -> Option<String> {
    let (workflows, parts) = parse();

    Some(
        parts
            .into_iter()
            .filter(|p| run_part(p, &workflows))
            .map(|p| p.x + p.m + p.a + p.s)
            .sum::<i32>()
            .to_string(),
    )
}

type Xmas = [Range<i32>; 4];

fn run(current: &str, workflows: &HashMap<String, Workflow>, [x, m, a, s]: Xmas) -> Vec<Xmas> {
    if let Some(workflow) = workflows.get(current) {
        let mut res: Vec<Xmas> = Vec::new();

        for r in &workflow.rules {
            let (mut x, mut m, mut a, mut s) = (x.clone(), m.clone(), a.clone(), s.clone());

            if let Rule::Condition(Condition { attr, cmp, val }, next) = r {
                match attr {
                    'x' => match cmp {
                        Cmp::Lt => {
                            res.append(&mut run(
                                next,
                                workflows,
                                [x.start..*val, m.clone(), a.clone(), s.clone()],
                            ));
                            x.start = *val;
                        }
                        Cmp::Gt => {
                            res.append(&mut run(
                                next,
                                workflows,
                                [*val..x.end, m.clone(), a.clone(), s.clone()],
                            ));
                            x.end = *val;
                        }
                    },
                    'm' => match cmp {
                        Cmp::Lt => {
                            res.append(&mut run(
                                next,
                                workflows,
                                [x.clone(), m.start..*val, a.clone(), s.clone()],
                            ));
                            m.start = *val;
                        }
                        Cmp::Gt => {
                            res.append(&mut run(
                                next,
                                workflows,
                                [x.clone(), *val..m.end, a.clone(), s.clone()],
                            ));
                            m.end = *val
                        }
                    },
                    'a' => match cmp {
                        Cmp::Lt => {
                            res.append(&mut run(
                                next,
                                workflows,
                                [x.clone(), m.clone(), a.start..*val, s.clone()],
                            ));
                            a.start = *val;
                        }
                        Cmp::Gt => {
                            res.append(&mut run(
                                next,
                                workflows,
                                [x.clone(), m.clone(), *val..a.end, s.clone()],
                            ));
                            a.end = *val
                        }
                    },
                    's' => match cmp {
                        Cmp::Lt => {
                            res.append(&mut run(
                                next,
                                workflows,
                                [x.clone(), m.clone(), a.clone(), s.start..*val],
                            ));
                            s.start = *val;
                        }
                        Cmp::Gt => {
                            res.append(&mut run(
                                next,
                                workflows,
                                [x.clone(), m.clone(), a.clone(), *val..s.end],
                            ));
                            s.end = *val
                        }
                    },
                    _ => panic!(),
                };
            }

            if let Rule::Default(next) = r {
                res.append(&mut run(next, workflows, [x, m, a, s]))
            }
        }

        res
    } else if current == "A" {
        vec![[x, m, a, s]]
    } else {
        Vec::new()
    }
}

fn overlaps(left: &Xmas, right: &Xmas) -> Option<Xmas> {
    let mut res = [0..0, 0..0, 0..0, 0..0];

    for (i, (l, r)) in left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| if l.end > r.end { (r, l) } else { (l, r) })
        .enumerate()
    {
        // dbg!(i, l, r);

        if l == r {
            res[i] = l.clone();
        } else {
            res[i] = std::cmp::max(r.start, l.start)..std::cmp::min(r.end, l.end);
        }

        if res[i].is_empty() {
            // dbg!("no overlap");
            return None;
        }
    }
    // dbg!(&res);

    Some(res)
}

pub fn two() -> Option<String> {
    let (workflows, _) = parse();

    let res = run("in", &workflows, [0..4000, 0..4000, 0..4000, 0..4000]);

    let combinations: Vec<Vec<&Xmas>> = res.iter().combinations(2).collect();

    dbg!(res.len(), combinations.len());

    let overlaps = combinations
        .into_iter()
        .filter_map(|c| overlaps(&c[0], &c[1]))
        .collect::<Vec<Xmas>>();

    dbg!(&overlaps);

    let over = overlaps
        .iter()
        .map(|x| x.iter().map(|r| r.len()).product::<usize>())
        .sum::<usize>();

    let res = res
        .iter()
        .map(|x| x.iter().map(|r| r.len()).product::<usize>())
        .sum::<usize>();

    dbg!(res, over, (res as f32 - over as f32) / 167409079868000.0);

    dbg!(4000_i64 * 4000 * 4000 * 4000);

    return Some((res - over).to_string());

    // TODO: Some kind of symbolic execution?
    //
    //
    // List of conditions that lead to A
    // let conditions: Vec<Vec<Condition>> = conditions_to_a("in", &workflows, Vec::new());

    // dbg!(conditions);

    None
}

fn conditions_to_a(
    workflow: &str,
    workflows: &HashMap<String, Workflow>,
    conditions: Vec<Condition>,
) -> Vec<Vec<Condition>> {
    dbg!(workflow);

    if let Some(workflow) = workflows.get(workflow) {
        let mut results = Vec::new();

        for rule in &workflow.rules {
            match rule {
                Rule::Condition(_, n) => dbg!(n),
                Rule::Default(n) => dbg!(n),
            };

            let mut res = match rule {
                Rule::Condition(c, next) => conditions_to_a(next, workflows, vec![c.clone()]),
                Rule::Default(next) => match next.as_str() {
                    "A" => return vec![conditions],
                    "R" => Vec::new(),
                    name => conditions_to_a(name, workflows, Vec::new()),
                },
            };

            res = res
                .iter_mut()
                .map(|cs| conditions.iter().chain(cs.iter()).cloned().collect())
                .collect();

            results.append(&mut res);
        }

        results
    } else {
        if workflow == "A" {
            vec![conditions]
        } else {
            Vec::new()
        }
    }
}

mod tests {}
