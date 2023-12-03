use std::collections::HashSet;

static INPUT: &'static str = include_str!("./input");

pub fn one() -> Option<String> {
    let input = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            (
                split.next().unwrap().to_string(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .flat_map(|(d, c)| {
            (0..c)
                .into_iter()
                .map(|_| d.clone())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for cmd in input {
        match cmd.as_str() {
            "U" => {
                head.1 += 1;
            }
            "R" => {
                head.0 += 1;
            }
            "D" => {
                head.1 -= 1;
            }
            "L" => {
                head.0 -= 1;
            }
            _ => panic!("{}", cmd),
        }

        let distance = distance(head, tail);

        let diff = (head.0 - tail.0, head.1 - tail.1);
        if distance < 1.5 {
        } else {
            tail.0 += diff.0.clamp(-1, 1);
            tail.1 += diff.1.clamp(-1, 1);
        }

        visited.insert(tail.clone());
    }

    Some(visited.len().to_string())
}

fn distance(head: (i32, i32), tail: (i32, i32)) -> f32 {
    (((head.0 - tail.0) * (head.0 - tail.0) + (head.1 - tail.1) * (head.1 - tail.1)) as f32).sqrt()
}

pub fn two() -> Option<String> {
    let input = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            (
                split.next().unwrap().to_string(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .flat_map(|(d, c)| {
            (0..c)
                .into_iter()
                .map(|_| d.clone())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>();

    let mut head = (0, 0);
    let mut tails = Vec::from([
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ]);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for cmd in input {
        match cmd.as_str() {
            "U" => {
                head.1 += 1;
            }
            "R" => {
                head.0 += 1;
            }
            "D" => {
                head.1 -= 1;
            }
            "L" => {
                head.0 -= 1;
            }
            _ => panic!("{}", cmd),
        }

        for i in 0..tails.len() {
            let prev = if i == 0 {
                head
            } else {
                *tails.get(i - 1).unwrap()
            };
            let tail = tails.get_mut(i).unwrap();
            let distance = distance(prev, tail.clone());

            let diff = (prev.0 - tail.0, prev.1 - tail.1);
            if distance < 1.5 {
            } else {
                tail.0 += diff.0.clamp(-1, 1);
                tail.1 += diff.1.clamp(-1, 1);
            }
        }

        visited.insert(tails.last().unwrap().clone());
    }

    Some(visited.len().to_string())
}
