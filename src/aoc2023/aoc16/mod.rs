use std::collections::HashSet;

static INPUT: &'static str = include_str!("./input");

fn parse() -> Vec<Vec<char>> {
    INPUT.lines().map(|l| l.chars().collect()).collect()
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Dir {
    R,
    L,
    U,
    D,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Ray {
    row: i32,
    col: i32,
    dir: Dir,
}

fn energize(start: Ray, grid: &Vec<Vec<char>>) -> usize {
    let mut rays = vec![start];

    let mut all_rays: HashSet<Ray> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some(ray) = rays.pop() {
        if all_rays.contains(&ray) {
            continue;
        }

        all_rays.insert(ray.clone());

        let height = grid.len() as i32;
        let width = grid.first().unwrap().len() as i32;

        if ray.row >= 0 && ray.col >= 0 && ray.row < height && ray.col < width {
            visited.insert((ray.row as usize, ray.col as usize));
        }

        if let Some(mut next) = step_ray(ray, &grid) {
            rays.append(&mut next);
        }
    }

    visited.len()
}

#[allow(dead_code)]
fn printv(visited: &HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) {
    let res = grid
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, _)| if visited.contains(&(r, c)) { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    dbg!(res);
}

pub fn one() -> Option<String> {
    let grid = parse();

    let start = Ray {
        row: 0,
        col: -1,
        dir: Dir::R,
    };

    Some(energize(start, &grid).to_string())
}

fn step_ray(ray: Ray, grid: &Vec<Vec<char>>) -> Option<Vec<Ray>> {
    let (row, col) = match ray.dir {
        Dir::R => (ray.row, ray.col + 1),
        Dir::L => (ray.row, ray.col - 1),
        Dir::U => (ray.row - 1, ray.col),
        Dir::D => (ray.row + 1, ray.col),
    };

    match (grid.get(row as usize)?.get(col as usize)?, ray.dir.clone()) {
        ('.', dir) => Some(vec![Ray { row, col, dir }]),
        ('-', Dir::R) | ('-', Dir::L) | ('|', Dir::D) | ('|', Dir::U) => Some(vec![Ray {
            row,
            col,
            dir: ray.dir,
        }]),

        ('/', Dir::R) | ('\\', Dir::L) => Some(vec![Ray {
            row,
            col,
            dir: Dir::U,
        }]),
        ('/', Dir::U) | ('\\', Dir::D) => Some(vec![Ray {
            row,
            col,
            dir: Dir::R,
        }]),
        ('\\', Dir::U) | ('/', Dir::D) => Some(vec![Ray {
            row,
            col,
            dir: Dir::L,
        }]),
        ('\\', Dir::R) | ('/', Dir::L) => Some(vec![Ray {
            row,
            col,
            dir: Dir::D,
        }]),
        ('-', Dir::D) | ('-', Dir::U) => Some(vec![
            Ray {
                row,
                col,
                dir: Dir::L,
            },
            Ray {
                row,
                col,
                dir: Dir::R,
            },
        ]),
        ('|', Dir::R) | ('|', Dir::L) => Some(vec![
            Ray {
                row,
                col,
                dir: Dir::U,
            },
            Ray {
                row,
                col,
                dir: Dir::D,
            },
        ]),
        (c, d) => panic!("Unexpected {c}, {d:?}"),
    }
}

pub fn two() -> Option<String> {
    let grid = parse();

    let height = grid.len();
    let width = grid.first().unwrap().len();

    let top = (0..width).map(|i| Ray {
        row: -1,
        col: i as i32,
        dir: Dir::D,
    });
    let right = (0..height).map(|i| Ray {
        row: i as i32,
        col: width as i32,
        dir: Dir::L,
    });
    let bottom = (0..width).map(|i| Ray {
        row: height as i32,
        col: i as i32,
        dir: Dir::U,
    });
    let left = (0..height).map(|i| Ray {
        row: i as i32,
        col: -1,
        dir: Dir::R,
    });

    Some(
        top.chain(right)
            .chain(bottom)
            .chain(left)
            .map(|r| energize(r, &grid))
            .max()
            .unwrap()
            .to_string(),
    )
}

mod tests {}
