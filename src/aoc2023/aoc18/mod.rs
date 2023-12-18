use std::{collections::HashMap, thread};

static INPUT: &'static str = include_str!("./example");

type Color = String;
type Grid = HashMap<(i32, i32), State>;

fn parse2() -> Vec<(char, i32, Color)> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            let col = split.nth(2).unwrap().replace(")", "").replace("(", "");

            let dir = match col.chars().last().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                s => panic!("unexpected last {s}"),
            };

            let len = i32::from_str_radix(&col.chars().skip(1).take(5).collect::<String>(), 16)
                .expect("parse");

            (dir, len, col)
        })
        .collect()
}

fn parse1() -> Vec<(char, i32, Color)> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            let dir = split.next().unwrap().chars().nth(0).unwrap();

            let len = split.next().unwrap().parse::<i32>().unwrap();

            let col = split.next().unwrap().replace(")", "").replace("(", "");

            (dir, len, col)
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Dirt,
    Hole(Color),
    Inside,
}

fn print(grid: &Grid) {
    let min_row = grid
        .keys()
        .map(|(row, _)| row.clone())
        .min()
        .unwrap_or_default();
    let min_col = grid
        .keys()
        .map(|(_, col)| col.clone())
        .min()
        .unwrap_or_default();
    let max_row = grid
        .keys()
        .map(|(row, _)| row.clone())
        .max()
        .unwrap_or_default();
    let max_col = grid
        .keys()
        .map(|(_, col)| col.clone())
        .max()
        .unwrap_or_default();

    let g = (min_row..=max_row)
        .map(|row| {
            (min_col..=max_col)
                .map(|col| {
                    if let Some((_, state)) = grid.iter().find(|((r, c), _)| r == &row && c == &col)
                    {
                        match state {
                            State::Dirt => '.',
                            State::Hole(_) => '#',
                            State::Inside => '#',
                        }
                    } else {
                        '?'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    dbg!(g);
}

fn get_grid(instructions: Vec<(char, i32, String)>) -> Grid {
    let mut row = 0;
    let mut col = 0;

    let mut grid = HashMap::new();

    for (dir, len, color) in instructions {
        match dir {
            'R' => {
                for c in col..=(col + len) {
                    grid.insert((row, c), State::Hole(color.to_string()));
                }

                col += len
            }
            'D' => {
                for r in row..=(row + len) {
                    grid.insert((r, col), State::Hole(color.to_string()));
                }

                row += len
            }
            'L' => {
                for c in (col - len)..=col {
                    grid.insert((row, c), State::Hole(color.to_string()));
                }

                col -= len
            }
            'U' => {
                for r in (row - len)..=row {
                    grid.insert((r, col), State::Hole(color.to_string()));
                }

                row -= len
            }
            s => panic!("unexpected {s}"),
        };
    }

    grid
}

fn fill(grid: &mut Grid, (row, col): (i32, i32)) {
    let current = grid.entry((row, col)).or_insert(State::Dirt);

    if let State::Dirt = current {
        grid.insert((row, col), State::Inside);

        fill(grid, (row + 1, col));
        fill(grid, (row.checked_sub(1).unwrap_or(0), col));
        fill(grid, (row, col + 1));
        fill(grid, (row, col.checked_sub(1).unwrap_or(0)));
    }
}

pub fn one() -> Option<String> {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(run_one)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap()
}

fn run_one() -> Option<String> {
    let mut grid = get_grid(parse1());

    fill(&mut grid, (1, 1));

    print(&grid);

    Some(
        grid.values()
            .filter(|c| match c {
                State::Dirt => false,
                State::Hole(_) => true,
                State::Inside => true,
            })
            .count()
            .to_string(),
    )
}

pub fn two() -> Option<String> {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(run_two)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap()
}

fn run_two() -> Option<String> {
    let mut grid = get_grid(parse2());

    fill(&mut grid, (0, 0));

    // print(&grid);

    Some(
        grid.values()
            .filter(|c| match c {
                State::Dirt => false,
                State::Hole(_) => true,
                State::Inside => true,
            })
            .count()
            .to_string(),
    )
}

mod tests {}
