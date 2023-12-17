use std::collections::HashMap;

static INPUT: &'static str = include_str!("./example");

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Node {
    row: i32,
    col: i32,
    loss: i32,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Dir {
    R,
    L,
    U,
    D,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Position {
    node: Node,
    dir: Dir,
    straight: i32,
}

fn parse() -> Vec<Node> {
    INPUT
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| Node {
                    row: row as i32,
                    col: col as i32,
                    loss: c.to_string().parse::<i32>().unwrap(),
                })
                .collect::<Vec<Node>>()
        })
        .collect()
}

fn next(pos: &Position, nodes: &Vec<Node>) -> Vec<Position> {
    let res = vec![
        (pos.node.row, pos.node.col + 1, Dir::R),
        (pos.node.row, pos.node.col - 1, Dir::L),
        (pos.node.row - 1, pos.node.col, Dir::U),
        (pos.node.row + 1, pos.node.col, Dir::D),
    ]
    .into_iter()
    // Remove straight if more than 3
    .filter(|n| {
        if pos.straight >= 3 {
            let res = n
                != &match pos.dir {
                    Dir::R => (pos.node.row, pos.node.col + 1, Dir::R),
                    Dir::L => (pos.node.row, pos.node.col - 1, Dir::L),
                    Dir::U => (pos.node.row - 1, pos.node.col, Dir::U),
                    Dir::D => (pos.node.row + 1, pos.node.col, Dir::D),
                };

            res
        } else {
            true
        }
    })
    // Remove reverse
    .filter(|(_, _, d)| match (d, &pos.dir) {
        (Dir::R, Dir::L) => false,
        (Dir::L, Dir::R) => false,
        (Dir::U, Dir::D) => false,
        (Dir::D, Dir::U) => false,
        _ => true,
    })
    // Remove out of bounds and map to position
    .filter_map(|(row, col, dir)| {
        let node = nodes.iter().find(|n| n.row == row && n.col == col)?;

        Some(Position {
            node: node.clone(),
            dir: dir.clone(),
            straight: if dir == pos.dir { pos.straight + 1 } else { 1 },
        })
    })
    .collect();

    res
}

pub fn one() -> Option<String> {
    let nodes = parse();

    let startr = Position {
        node: nodes.first().unwrap().clone(),
        dir: Dir::R,
        straight: 1,
    };
    let startd = Position {
        node: nodes.first().unwrap().clone(),
        dir: Dir::D,
        straight: 1,
    };

    let goal = nodes.last().unwrap().clone();

    let mut losses = HashMap::new();
    losses.insert(startr.node.clone(), 0);
    losses.insert(startd.node.clone(), 0);

    let mut prev: HashMap<Position, Position> = HashMap::new();

    // Problem: Dijkstra may not work?
    let mut to_visit = vec![startr.clone(), startd.clone()];

    while let Some(current) = to_visit.pop() {
        let current_loss = losses.get(&current.node).unwrap_or(&i32::MAX).clone();

        for next in next(&current, &nodes) {
            let next_loss = losses.get(&next.node).unwrap_or(&i32::MAX);

            if current_loss + next.node.loss < *next_loss {
                losses.insert(next.node.clone(), current_loss + next.node.loss);
                to_visit.push(next.clone());
                prev.insert(next.clone(), current.clone());
                print(path_for(&next.node, &prev));
            }
        }
    }

    let path = path_for(&goal, &prev);
    dbg!(path.iter().map(|p| p.node.loss).sum::<i32>());
    print(path);

    Some(
        losses
            .into_iter()
            .find(|(p, _)| p == &goal)
            .unwrap()
            .1
            .to_string(),
    )
}

fn path_for(node: &Node, prev: &HashMap<Position, Position>) -> Vec<Position> {
    let mut current = prev.keys().find(|k| k.node == *node).unwrap();

    let mut path: Vec<Position> = vec![current.clone()];

    while let Some(p) = prev.get(current) {
        current = p;
        path.push(p.clone());
    }

    path.into_iter().rev().skip(1).rev().collect()
}

fn print(path: Vec<Position>) {
    let height = 13;
    let width = 13;

    let rows = (0..height)
        .map(|r| {
            let row = (0..width)
                .map(|c| {
                    if let Some(pos) = path.iter().find(|p| p.node.row == r && p.node.col == c) {
                        pos.node.loss.to_string().chars().nth(0).unwrap()
                        // pos.straight.to_string().chars().nth(0).unwrap()
                        // match pos.dir {
                        //     Dir::R => '>',
                        //     Dir::L => '<',
                        //     Dir::U => '^',
                        //     Dir::D => 'v',
                        // }
                    } else {
                        '.'
                    }
                })
                .collect::<String>();

            row
        })
        .collect::<Vec<String>>();

    dbg!(rows);
}

pub fn two() -> Option<String> {
    None
}

mod tests {}
