use std::collections::HashMap;

static INPUT: &'static str = include_str!("./input");

fn parse() -> Vec<String> {
    INPUT.lines().map(|l| l.to_string()).collect()
}

fn rotate(g: Vec<String>) -> Vec<String> {
    let transposed = transpose(&g.iter().map(|l| l.chars().collect()).collect())
        .into_iter()
        .map(|l| l.iter().rev().collect::<String>())
        .collect();

    transposed
}

fn transpose(inner: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..inner[0].len())
        .map(|i| {
            inner
                .iter()
                .map(|inn| inn[i].clone())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn slide_right(s: String) -> String {
    let mut s = s;
    while s.contains("O.") {
        s = s.replace("O.", ".O");
    }
    s
}

pub fn one() -> Option<String> {
    let res: Vec<String> = rotate(parse()).into_iter().map(slide_right).collect();

    let r: usize = weight(&rotate(rotate(rotate(res))));

    Some(r.to_string())
}

fn weight(grid: &Vec<String>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, l)| (i + 1) * (l.chars().filter(|c| c == &'O').count()))
        .sum()
}

pub fn two() -> Option<String> {
    let mut grid = parse();

    let mut grids: HashMap<Vec<String>, usize> = HashMap::new();

    let full_iterations = 1_000_000_000;

    for i in 0..full_iterations {
        grid = grid_after_full_iteration(grid);

        if let Some(o) = grids.get(&grid) {
            let remaining = (full_iterations - i) % (i - o);

            for _ in 1..remaining {
                grid = grid_after_full_iteration(grid);
            }

            return Some(weight(&grid).to_string());
        } else {
            grids.insert(grid.clone(), i);
        }
    }

    Some(weight(&grid).to_string())
}

fn grid_after_full_iteration(grid: Vec<String>) -> Vec<String> {
    let mut grid = grid;

    // north
    grid = rotate(grid).into_iter().map(slide_right).collect();
    grid = rotate(grid);

    // west
    grid = grid.into_iter().map(slide_right).collect();
    grid = grid;

    // south
    grid = rotate(grid).into_iter().map(slide_right).collect();
    grid = rotate(grid);

    // east
    grid = grid.into_iter().map(slide_right).collect();

    grid
}

mod tests {}
