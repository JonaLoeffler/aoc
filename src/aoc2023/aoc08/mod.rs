use num::integer::lcm;
use std::collections::HashMap;

static INPUT: &'static str = include_str!("./input");

#[derive(Debug)]
enum Dir {
    L,
    R,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'L' => Dir::L,
            'R' => Dir::R,
            _default => panic!(),
        }
    }
}

type Node = String;

fn parse() -> (Vec<Dir>, HashMap<Node, (Node, Node)>) {
    let dirs = INPUT
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| -> Dir { c.into() })
        .collect();

    let nodes = INPUT
        .lines()
        .skip(2)
        .map(|l| {
            let mut split = l.split(" = ");
            let start = split.next().unwrap();

            let next = split.next().unwrap();

            let left = next.split(", ").nth(0).unwrap().replace("(", "");
            let right = next.split(", ").nth(1).unwrap().replace(")", "");

            (start.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();

    (dirs, nodes)
}

pub fn one() -> Option<String> {
    let (dirs, nodes) = parse();

    let mut count = 0;
    let mut current = "AAA";
    for dir in dirs.iter().cycle() {
        if let Some(next) = nodes.get(current) {
            current = match dir {
                Dir::L => &next.0,
                Dir::R => &next.1,
            };
            count += 1;
        }

        if current == "ZZZ" {
            break;
        }
    }

    Some(count.to_string())
}

pub fn two() -> Option<String> {
    let (dirs, nodes) = parse();

    let mut count = 0;
    let mut current: Vec<Node> = nodes
        .keys()
        .filter(|k| k.chars().nth(2).is_some_and(|c| c == 'A'))
        .cloned()
        .collect();

    let mut least: Vec<usize> = (0..current.len()).map(|_| 0).collect();

    for dir in dirs.iter().cycle() {
        current = current
            .iter()
            .map(|cur| {
                let n = nodes.get(cur).unwrap();

                (match dir {
                    Dir::L => &n.0,
                    Dir::R => &n.1,
                })
                .to_string()
            })
            .collect();

        count += 1;

        for (i, c) in current.iter().enumerate() {
            let l = least.get_mut(i).unwrap();
            if c.chars().nth(2) == Some('Z') && l == &0 {
                *l = count
            }
        }

        if least.iter().all(|l| l > &0) {
            break;
        }
    }

    Some(least.into_iter().reduce(lcm)?.to_string())
}

mod tests {}
