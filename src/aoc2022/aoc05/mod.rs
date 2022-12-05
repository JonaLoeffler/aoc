use std::collections::BTreeMap;

static INPUT: &'static str = include_str!("./input");

type Command = (usize, usize, usize);
type Stacks = BTreeMap<usize, Vec<String>>;

fn input() -> (Stacks, Vec<Command>) {
    let stackstring = INPUT.split("\n\n").nth(0).unwrap();
    let commandstring = INPUT.split("\n\n").nth(1).unwrap();

    let mut stacks = BTreeMap::new();

    for item in stackstring
        .lines()
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
    {
        let crates: Vec<String> = Vec::new();
        stacks.insert(item.parse::<usize>().unwrap(), crates);
    }

    let replaced = stackstring
        .replace("     ", " [_] ")
        .replace("     ", " [_] ")
        .replace("    ", "[_] ");

    for line in replaced.replace("     ", " [_] ").lines().rev().skip(1) {
        for (i, s) in line.split(" ").enumerate() {
            if s != "[_]" {
                let list = stacks.get_mut(&(&i + 1)).unwrap();

                list.push(s.to_string());
            }
        }
    }

    let commands = commandstring
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "")
        .lines()
        .map(|s| {
            let mut iter = s.split(" ");

            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<Command>>();

    (stacks, commands)
}

pub fn one() -> String {
    let (mut stacks, commands) = input();

    for (a, b, c) in commands {
        let popped = {
            let mut popped = Vec::new();
            let from = stacks.get_mut(&b).unwrap();
            for _ in 0..a {
                popped.push(from.pop().unwrap());
            }
            popped
        };

        let to = stacks.get_mut(&c).unwrap();

        for c in popped.iter() {
            to.push(c.to_string());
        }
    }

    stacks
        .iter()
        .map(|(_, v)| v.last().unwrap().to_string())
        .collect::<String>()
        .replace("[", "")
        .replace("]", "")
}

pub fn two() -> String {
    let (mut stacks, commands) = input();

    for (a, b, c) in commands {
        let popped = {
            let mut popped = Vec::new();
            let from = stacks.get_mut(&b).unwrap();
            for _ in 0..a {
                popped.push(from.pop().unwrap());
            }
            popped
        };

        let to = stacks.get_mut(&c).unwrap();

        for c in popped.iter().rev() {
            to.push(c.to_string());
        }
    }

    stacks
        .iter()
        .map(|(_, v)| v.last().unwrap().to_string())
        .collect::<String>()
        .replace("[", "")
        .replace("]", "")
}
