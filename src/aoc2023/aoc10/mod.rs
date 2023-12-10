use std::collections::HashMap;

static INPUT: &'static str = include_str!("./example");

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Node {
    char: char,
    row: usize,
    col: usize,
}

fn parse() -> (Node, HashMap<Node, Vec<Node>>) {
    let mut res = HashMap::new();

    let mut lines: Vec<String> = INPUT.lines().map(|s| s.to_string()).collect();
    let binding = (0..lines.get(0).unwrap().len())
        .map(|_| ".")
        .collect::<String>();

    lines.insert(0, binding.clone());
    lines.push(binding.clone());

    lines = lines.into_iter().map(|l| format!(".{l}.")).collect();

    let mut start = None;

    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let vals = match char {
                '.' => continue,
                '-' => vec![(row, col - 1), (row, col + 1)],
                '|' => vec![(row - 1, col), (row + 1, col)],
                'L' => vec![(row - 1, col), (row, col + 1)],
                'J' => vec![(row, col - 1), (row - 1, col)],
                'F' => vec![(row + 1, col), (row, col + 1)],
                '7' => vec![(row, col - 1), (row + 1, col)],
                'S' => vec![
                    (row - 1, col),
                    (row + 1, col),
                    (row, col - 1),
                    (row, col + 1),
                ],
                _default => panic!(),
            };

            if char == 'S' {
                start = Some(Node { char, row, col })
            }

            res.insert(
                Node { char, row, col },
                vals.into_iter()
                    .filter_map(|(r, c)| {
                        let next = lines.get(r).unwrap().chars().nth(c).unwrap();

                        let up = vec!['|', 'L', 'J', 'S'];
                        let down = vec!['|', 'F', '7', 'S'];
                        let left = vec!['-', 'J', '7', 'S'];
                        let right = vec!['-', 'F', 'L', 'S'];

                        let valid = match (r as i32 - row as i32, c as i32 - col as i32) {
                            // down
                            (1, 0) => down.contains(&char) && up.contains(&next),
                            // up
                            (-1, 0) => up.contains(&char) && down.contains(&next),
                            // right
                            (0, 1) => right.contains(&char) && left.contains(&next),
                            // left
                            (0, -1) => left.contains(&char) && right.contains(&next),
                            a => panic!("{a:?}"),
                        };

                        match valid {
                            true => Some(Node {
                                char: next,
                                row: r,
                                col: c,
                            }),
                            false => None,
                        }
                    })
                    .filter(|n| n.char != '.')
                    .collect(),
            );
        }
    }

    (start.unwrap(), res)
}

fn visited(nodes: HashMap<Node, i32>) -> Vec<Vec<char>> {
    let rows = nodes.keys().map(|n| n.row).max().unwrap_or_default();
    let cols = nodes.keys().map(|n| n.col).max().unwrap_or_default();

    let mut out: Vec<Vec<char>> = (0..rows + 2)
        .map(|_| (0..cols + 2).map(|_| '.').collect::<Vec<char>>())
        .collect();

    for (node, _) in nodes {
        let n = out.get_mut(node.row).unwrap().get_mut(node.col).unwrap();
        *n = 'X';
    }

    out
}

pub fn one() -> Option<String> {
    let (start, next) = parse();

    let mut distances: HashMap<Node, i32> = next
        .keys()
        .cloned()
        .map(|k| (k.clone(), i32::MAX))
        .collect();

    distances.insert(start.clone(), 0);

    let mut to_visit = vec![start.clone()];

    while let Some(current) = to_visit.pop() {
        let current_distance = distances.get(&current).unwrap_or(&i32::MAX).clone();

        for next in next.get(&current).unwrap_or(&vec![]) {
            let next_distance = distances.get(&next).unwrap_or(&i32::MAX);

            if current_distance + 1 < *next_distance {
                distances.insert(next.clone(), current_distance + 1);
                to_visit.push(next.clone());
            }
        }
    }

    distances
        .values()
        .filter(|v| v != &&i32::MAX)
        .max()
        .map(|c| c.to_string())
}

pub fn two() -> Option<String> {
    let (start, next) = parse();

    let mut distances: HashMap<Node, i32> = next
        .keys()
        .cloned()
        .map(|k| (k.clone(), i32::MAX))
        .collect();

    distances.insert(start.clone(), 0);

    let mut to_visit = vec![start.clone()];

    while let Some(current) = to_visit.pop() {
        let current_distance = distances.get(&current).unwrap_or(&i32::MAX).clone();

        for next in next.get(&current).unwrap_or(&vec![]) {
            let next_distance = distances.get(&next).unwrap_or(&i32::MAX);

            if current_distance + 1 < *next_distance {
                distances.insert(next.clone(), current_distance + 1);
                to_visit.push(next.clone());
            }
        }
    }

    let mut inner = visited(distances.clone());

    // Iterator over rows
    for (row, line) in inner.clone().iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            let range = &line[0..col];
            let is_inside = range.iter().filter(|c| c == &&'X').count() % 2 == 1;

            if is_inside && char == &'.' {
                let n = inner.get_mut(row).unwrap().get_mut(col).unwrap();
                *n = 'I';
            }
        }
    }

    // iterate over cols
    for (row, line) in inner.clone().iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            let range = &line[0..col];
            let is_inside = range.iter().filter(|c| c == &&'X').count() % 2 == 1;

            if is_inside && char == &'.' {
                let n = inner.get_mut(row).unwrap().get_mut(col).unwrap();
                *n = 'I';
            }
        }
    }

    // println!(
    //     "{}\n",
    //     inner
    //         .iter()
    //         .map(|l| l.iter().map(|c| c.to_string()).collect::<Vec<String>>())
    //         .map(|l| l.join(""))
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );

    Some(
        inner
            .iter()
            .map(|r| r.iter().filter(|c| c == &&'I').count())
            .sum::<usize>()
            .to_string(),
    )
}

mod tests {}
