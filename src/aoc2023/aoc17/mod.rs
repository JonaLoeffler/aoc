use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

static INPUT: &'static str = include_str!("./input");

#[derive(PartialEq, Eq, Debug, Clone, Hash, Ord, PartialOrd)]
struct Node {
    row: i32,
    col: i32,
    loss: i32,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Ord, PartialOrd)]
enum Dir {
    R,
    L,
    U,
    D,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Ord, PartialOrd)]
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
    vec![
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
    .collect()
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

    let mut losses: HashMap<Position, i32> = HashMap::new();
    losses.insert(startr.clone(), 0);
    losses.insert(startd.clone(), 0);

    // let mut prev: HashMap<Position, Position> = HashMap::new();

    let mut to_visit: BinaryHeap<Reverse<(i32, Position)>> = BinaryHeap::new();
    to_visit.push(Reverse((0, startr)));
    to_visit.push(Reverse((0, startd)));

    while let Some(Reverse((curr_cost, current))) = to_visit.pop() {
        if current.node == goal {
            break;
        }

        let current_loss = losses.get(&current).unwrap_or(&i32::MAX).clone();

        if curr_cost > current_loss {
            continue;
        }

        for next in next(&current, &nodes) {
            let next_loss = losses.get(&next).unwrap_or(&i32::MAX);

            let loss = current_loss + next.node.loss;
            if loss < *next_loss {
                losses.insert(next.clone(), loss);
                to_visit.push(Reverse((loss, next.clone())));
                // prev.insert(next.clone(), current.clone());
            }
        }
    }

    // let path = path_for(&goal, &prev);
    // dbg!(path.iter().map(|p| p.node.loss).sum::<i32>());
    // print(path);

    Some(
        losses
            .into_iter()
            .filter(|(p, _)| p.node == goal)
            .map(|l| l.1)
            .min()?
            .to_string(),
    )
}

// fn path_for(node: &Node, prev: &HashMap<Position, Position>) -> Vec<Position> {
//     let reach_goal: Vec<Position> = prev.keys().cloned().filter(|k| k.node == *node).collect();
//     dbg!(&reach_goal);

//     let mut current = reach_goal.first().unwrap();

//     let mut path: Vec<Position> = vec![current.clone()];

//     while let Some(p) = prev.get(current) {
//         current = p;
//         path.push(p.clone());
//     }

//     path.into_iter().rev().skip(1).rev().collect()
// }

// fn print(path: Vec<Position>) {
//     let height = 13;
//     let width = 13;

//     let rows = (0..height)
//         .map(|r| {
//             let row = (0..width)
//                 .map(|c| {
//                     if let Some(pos) = path.iter().find(|p| p.node.row == r && p.node.col == c) {
//                         // pos.node.loss.to_string().chars().nth(0).unwrap()
//                         // pos.straight.to_string().chars().nth(0).unwrap()
//                         match pos.dir {
//                             Dir::R => '>',
//                             Dir::L => '<',
//                             Dir::U => '^',
//                             Dir::D => 'v',
//                         }
//                     } else {
//                         '.'
//                     }
//                 })
//                 .collect::<String>();

//             row
//         })
//         .collect::<Vec<String>>();

//     dbg!(rows);
// }

pub fn two() -> Option<String> {
    None
}

mod tests {}
