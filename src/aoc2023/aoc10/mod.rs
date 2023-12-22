use std::collections::HashMap;

static INPUT: &'static str = include_str!("./input");

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Node {
    char: char,
    row: usize,
    col: usize,
}

// TODO: use shoelace-formula with pick's theorem

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
        *n = node.char;
    }

    out
}

pub fn one() -> Option<String> {
    let (start, next) = parse();

    let mut distances = HashMap::new();

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

    let mut distances = HashMap::new();

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
    // print(&inner);

    // Iterator over rows
    inner = inner.iter().map(fill_inner).collect();

    let mut transposed = transpose(visited(distances.clone()).clone());

    // iterate over cols
    transposed = transposed.iter().map(fill_inner).collect();
    transposed = transpose(transposed.clone());

    // print(&inner);
    // print(&transposed);

    let mut both: Vec<Vec<char>> = inner
        .iter()
        .zip(transposed.iter())
        .map(|(il, tl)| {
            il.iter()
                .zip(tl.iter())
                .map(|(ic, tc)| match (ic, tc) {
                    ('I', 'I') => 'I',
                    ('I', _) => '.',
                    (_, 'I') => '.',
                    _ => ic.to_owned(),
                })
                .collect()
        })
        .map(replace_outer)
        .collect();

    both = transpose(transpose(both).into_iter().map(replace_outer).collect());

    // print(&both);

    Some(
        both.iter()
            .map(|r| r.iter().filter(|c| c == &&'I').count())
            .sum::<usize>()
            .to_string(),
    )
}

fn transpose(inner: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..inner[0].len())
        .map(|i| {
            inner
                .iter()
                .map(|inn| inn[i].clone())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn fill_inner(line: &Vec<char>) -> Vec<char> {
    let mut is_inside = false;

    line.iter()
        .enumerate()
        .map(|(col, char)| {
            let change = vec!['|', '-', 'L', 'F', 'J', '7'];
            let prev = line
                .get((col as i32 - 1).try_into().unwrap_or(100000))
                .unwrap_or(&'X');

            if change.contains(prev) && char == &'.' {
                is_inside = !is_inside;
            } else if change.contains(char) {
                is_inside = false;
            }

            let last_x_index = line
                .iter()
                .enumerate()
                .filter(|(_, c)| c != &&'.')
                .last()
                .unwrap_or((0, &'.'))
                .0;

            if is_inside && char == &'.' && col < last_x_index {
                'I'
            } else {
                *char
            }
        })
        .collect()
}

fn replace_outer(l: Vec<char>) -> Vec<char> {
    let mut line: String = l.iter().collect();

    while line.contains("I.") {
        line = line.replace("I.", "..");
    }

    while line.contains(".I") {
        line = line.replace(".I", "..");
    }

    line.chars().collect()
}

#[allow(unused)]
fn print(inner: &Vec<Vec<char>>) {
    println!(
        "{}\n",
        inner
            .iter()
            .map(|l| l.iter().map(|c| c.to_string()).collect::<Vec<String>>())
            //
            .map(|l| l
                .join("")
                .replace("-", "─")
                .replace("|", "│")
                .replace("F", "╭")
                .replace("7", "╮")
                .replace("L", "╰")
                .replace("J", "╯"))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

mod tests {
    #[test]
    fn test_fill_inner() {
        let res = super::fill_inner(&".7|L.FL.FL-.".chars().collect());

        assert_eq!(res, ".7|LIFLIFL-.".chars().collect::<Vec<char>>())
    }

    #[test]
    fn test_replace_outer() {
        let res = super::replace_outer(".......................L-7L-J|||F-JFJFJL7F--JF7F7L7LJ.LJLJI||..IF7F7I||FJ|FJFJ||ILJ||F--J|F-J|FJF7F7|ILJF7....L7L-7...........................".chars().collect());

        assert_eq!(res, ".......................L-7L-J|||F-JFJFJL7F--JF7F7L7LJ.LJLJI||...F7F7I||FJ|FJFJ||ILJ||F--J|F-J|FJF7F7|ILJF7....L7L-7...........................".chars().collect::<Vec<char>>());
    }
}
