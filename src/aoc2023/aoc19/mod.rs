use std::collections::HashMap;

use iter_progress::ProgressableIter;
use itertools::Itertools;

static INPUT: &'static str = include_str!("./input");

#[derive(Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
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

#[derive(Debug)]
struct Condition {
    attr: char,
    cmp: Cmp,
    val: u16,
}

#[derive(Debug)]
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
                            .parse::<u16>()
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
                .parse::<u16>()
                .unwrap();
            let m = split
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse::<u16>()
                .unwrap();
            let a = split
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse::<u16>()
                .unwrap();
            let s = split
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse::<u16>()
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

    let res: usize = parts
        .into_iter()
        .filter(|p| run_part(p, &workflows))
        .map(|p| p.x as usize + p.m as usize + p.a as usize + p.s as usize)
        .sum();

    Some(res.to_string())
}

pub fn two() -> Option<String> {
    let (workflows, _) = parse();

    // TODO: Some kind of symbolic execution?
    let res: usize = (0..4)
        .map(|_| 0..4000)
        .multi_cartesian_product()
        .progress()
        .filter_map(|(state, v)| {
            let p = Part {
                x: v[0],
                m: v[1],
                a: v[2],
                s: v[3],
            };

            state.do_every_n_sec(1., |state| {
                println!(
                    "{}% the way though, and doing {} per sec. {:#?}",
                    state.percent().unwrap_or_default(),
                    state.rate(),
                    state.eta().unwrap()
                );
            });

            if run_part(&p, &workflows) {
                Some(p)
            } else {
                None
            }
        })
        .count();

    Some(res.to_string())
}

mod tests {}
