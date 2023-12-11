use itertools::Itertools;

static INPUT: &'static str = include_str!("./input");

#[derive(Debug)]
struct Galaxy {
    #[allow(unused)]
    num: usize,
    row: usize,
    col: usize,
}

fn parse(empty_factor: usize) -> Vec<Galaxy> {
    let mut expanded: Vec<Vec<char>> = Vec::new();
    let mut empty_rows = Vec::new();
    for (i, line) in INPUT.lines().enumerate() {
        if !line.contains("#") {
            empty_rows.push(i);
        }
        expanded.push(line.chars().collect());
    }

    let transposed: Vec<Vec<char>> = (0..expanded[0].len())
        .map(|i| {
            expanded
                .iter()
                .map(|inn| inn[i].clone())
                .collect::<Vec<char>>()
        })
        .collect();

    let mut empty_cols = Vec::new();
    for (i, line) in transposed.clone().iter().enumerate() {
        if !line.contains(&'#') {
            empty_cols.push(i);
        }
    }

    let final_list: Vec<Vec<char>> = (0..transposed[0].len())
        .map(|i| {
            transposed
                .iter()
                .map(|inn| inn[i].clone())
                .collect::<Vec<char>>()
        })
        .collect();

    let mut num = 0;
    final_list
        .iter()
        .enumerate()
        .flat_map(|(row, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(col, c)| match c {
                    '#' => {
                        num += 1;

                        Some(Galaxy {
                            num,
                            row: row.clone()
                                + empty_rows.iter().filter(|r| r < &&row).count()
                                    * (empty_factor - 1),
                            col: col.clone()
                                + empty_cols.iter().filter(|c| c < &&col).count()
                                    * (empty_factor - 1),
                        })
                    }
                    _ => None,
                })
                .collect::<Vec<Galaxy>>()
        })
        .collect()
}

pub fn one() -> Option<String> {
    Some(
        parse(2)
            .iter()
            .combinations(2)
            .map(|gs| {
                let a = gs.get(0).unwrap();
                let b = gs.get(1).unwrap();

                (a.row as i64 - b.row as i64).abs() + (a.col as i64 - b.col as i64).abs()
            })
            .sum::<i64>()
            .to_string(),
    )
}

pub fn two() -> Option<String> {
    Some(
        parse(1000000)
            .iter()
            .combinations(2)
            .map(|gs| {
                let a = gs.get(0).unwrap();
                let b = gs.get(1).unwrap();

                (a.row as i64 - b.row as i64).abs() + (a.col as i64 - b.col as i64).abs()
            })
            .sum::<i64>()
            .to_string(),
    )
}

#[allow(unused)]
fn print(inner: &Vec<Vec<char>>) {
    println!(
        "{}\n",
        inner
            .iter()
            .map(|l| l.iter().map(|c| c.to_string()).collect::<Vec<String>>())
            .map(|l| l.join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

mod tests {}
