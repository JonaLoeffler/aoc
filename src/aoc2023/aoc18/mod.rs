use std::thread;

static INPUT: &'static str = include_str!("./input");

type Color = String;

fn parse() -> Vec<(char, usize, Color)> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            let dir = split.next().unwrap().chars().nth(0).unwrap();

            let len = split.next().unwrap().parse::<usize>().unwrap();

            let col = split.next().unwrap().replace(")", "").replace("(", "");

            (dir, len, col)
        })
        .collect()
}

#[derive(Debug)]
enum Tile {
    Dirt,
    Hole(Color),
    Outside,
}

#[allow(dead_code)]
fn print(grid: &Vec<Vec<Tile>>) {
    let g = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|t| match t {
                    Tile::Dirt => '.',
                    Tile::Hole(_) => '#',
                    Tile::Outside => '_',
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    dbg!(g);
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
    let size = 800;
    // let size = 100;

    let mut grid: Vec<Vec<Tile>> = (0..size)
        .map(|_| (0..size).map(|_| Tile::Dirt).collect())
        .collect();

    let mut row = size / 2;
    let mut col = size / 2;

    for (dir, len, color) in parse() {
        match dir {
            'R' => {
                for c in col..=(col + len) {
                    grid[row][c] = Tile::Hole(color.to_string());
                }

                col += len
            }
            'D' => {
                for r in row..=(row + len) {
                    grid[r][col] = Tile::Hole(color.to_string());
                }

                row += len
            }
            'L' => {
                for c in (col - len)..=col {
                    grid[row][c] = Tile::Hole(color.to_string());
                }

                col -= len
            }
            'U' => {
                for r in (row - len)..=row {
                    grid[r][col] = Tile::Hole(color.to_string());
                }

                row -= len
            }
            s => panic!("unexpected {s}"),
        };
    }
    fill(&mut grid, (0, 0));

    Some(
        grid.iter()
            .flat_map(|l| {
                l.iter()
                    .filter(|c| match c {
                        Tile::Dirt => true,
                        Tile::Hole(_) => true,
                        Tile::Outside => false,
                    })
                    .collect::<Vec<&Tile>>()
            })
            .count()
            .to_string(),
    )
}

fn fill(grid: &mut Vec<Vec<Tile>>, (row, col): (usize, usize)) {
    if let Tile::Dirt = grid
        .get(row)
        .unwrap_or(&vec![])
        .get(col)
        .unwrap_or(&Tile::Hole("".to_string()))
    {
        grid[row][col] = Tile::Outside;
        fill(grid, (row + 1, col));
        fill(grid, (row.checked_sub(1).unwrap_or(0), col));
        fill(grid, (row, col + 1));
        fill(grid, (row, col.checked_sub(1).unwrap_or(0)));
    }
}

pub fn two() -> Option<String> {
    None
}

mod tests {}
