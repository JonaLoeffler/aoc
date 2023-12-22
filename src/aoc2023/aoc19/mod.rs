use std::{collections::HashMap, ops::Range};

static INPUT: &'static str = include_str!("./input");

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

        let (mut x, mut m, mut a, mut s) = (x.clone(), m.clone(), a.clone(), s.clone());

        for r in &workflow.rules {
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
                                [*val + 1..x.end, m.clone(), a.clone(), s.clone()],
                            ));
                            x.end = *val + 1;
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
                                [x.clone(), *val + 1..m.end, a.clone(), s.clone()],
                            ));
                            m.end = *val + 1;
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
                                [x.clone(), m.clone(), *val + 1..a.end, s.clone()],
                            ));
                            a.end = *val + 1;
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
                                [x.clone(), m.clone(), a.clone(), *val + 1..s.end],
                            ));
                            s.end = *val + 1;
                        }
                    },
                    _ => panic!(),
                };
            }

            if let Rule::Default(next) = r {
                res.append(&mut run(
                    next,
                    workflows,
                    [x.clone(), m.clone(), a.clone(), s.clone()],
                ))
            }
        }

        res
    } else if current == "A" {
        vec![[x, m, a, s]]
    } else {
        Vec::new()
    }
}

pub fn two() -> Option<String> {
    let (workflows, _) = parse();

    Some(
        run("in", &workflows, [1..4001, 1..4001, 1..4001, 1..4001])
            .iter()
            .map(|x| x.iter().map(|r| r.len()).product::<usize>())
            .sum::<usize>()
            .to_string(),
    )
}

mod tests {}
