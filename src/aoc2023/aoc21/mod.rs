use std::collections::HashSet;

use crate::aoc2023;

static INPUT: &'static str = include_str!("./example");

fn parse() -> Vec<Vec<char>> {
    INPUT.lines().map(|l| l.chars().collect()).collect()
}

#[allow(dead_code)]
fn print(grid: &Vec<Vec<char>>, lots: &HashSet<(usize, usize)>) {
    println!("");
    grid.iter().enumerate().for_each(|(row, l)| {
        let line = l
            .iter()
            .enumerate()
            .map(|(col, c)| if lots.contains(&(row, col)) { 'O' } else { *c })
            .collect::<String>();

        println!("{}", line);
    })
}

fn clamp_to_size(index: i64, size: i64) -> i64 {
    ((index % size) + size) % size
}

fn get(row: i64, col: i64, grid: &Vec<Vec<char>>, finite: bool) -> char {
    let size = grid.len() as i64;

    if finite && (row != clamp_to_size(row, size) || col != clamp_to_size(col, size)) {
        return '#';
    }

    let row = clamp_to_size(row, size);
    let col = clamp_to_size(col, size);

    *grid.get(row as usize).unwrap().get(col as usize).unwrap()
}

fn walk(grid: &Vec<Vec<char>>, steps: usize, finite: bool) -> usize {
    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, c)| (row, col, c.to_owned()))
                .collect::<Vec<(usize, usize, char)>>()
        })
        .find(|(_, _, c)| c == &'S')
        .unwrap();

    let mut lots: HashSet<(i64, i64)> = HashSet::new();
    lots.insert((start.0 as i64, start.1 as i64));

    let mut pattern: Vec<i64> = Vec::new();

    for step in 1..=steps {
        // print(&grid, &lots);

        let mut next = HashSet::new();

        for (row, col) in lots.clone() {
            if vec!['.', 'S'].contains(&get(row + 1, col, &grid, finite)) {
                next.insert((row + 1, col.clone()));
            }

            if vec!['.', 'S'].contains(&get(row - 1, col, &grid, finite)) {
                next.insert((row - 1, col.clone()));
            }

            if vec!['.', 'S'].contains(&get(row, col + 1, &grid, finite)) {
                next.insert((row.clone(), col + 1));
            }

            if vec!['.', 'S'].contains(&get(row, col - 1, &grid, finite)) {
                next.insert((row.clone(), col - 1));
            }
        }

        if step % grid.len() == 0 {
            pattern.push(next.len() as i64);

            dbg!(&pattern);

            let mut res: Vec<i64> = pattern.windows(2).skip(3).map(|w| w[1] - w[0]).collect();
            // dbg!(&res);

            res = res.windows(2).map(|w| w[1] - w[0]).collect();

            // dbg!(&res);

            res = res.windows(2).map(|w| w[1] - w[0]).collect();

            dbg!(&res);

            let input: Vec<i64> = pattern.iter().skip(3).cloned().collect();
            if res.iter().filter(|s| s == &&0).count() > 3 {
                let mut curr = step;

                while curr < steps {
                    // dbg!(&pattern);

                    pattern.push(aoc2023::aoc09::line(input.clone()));
                    curr += grid.len();

                    // dbg!(other);
                }

                // dbg!(&pattern);

                let mut res: Vec<i64> = pattern.windows(2).map(|w| w[1] - w[0]).collect();
                // dbg!(&res);

                res = res.windows(2).map(|w| w[1] - w[0]).collect();

                // dbg!(&res);

                res = res.windows(2).map(|w| w[1] - w[0]).collect();

                // dbg!(&res);

                return *pattern.last().unwrap() as usize;

                let remaining = (steps - step) / grid.len();

                dbg!(grid.len(), step, steps, remaining);

                return 0;
            }
        }

        lots = next;
    }

    lots.len()
}

pub fn one() -> Option<String> {
    Some(walk(&parse(), 64, true).to_string())
}

pub fn two() -> Option<String> {
    Some(walk(&parse(), 500, false).to_string())
}

mod tests {
    #[test]
    fn test_clamp_to_size() {
        assert_eq!(0, super::clamp_to_size(-11, 11));
        assert_eq!(1, super::clamp_to_size(-10, 11));
        assert_eq!(2, super::clamp_to_size(-9, 11));
        assert_eq!(3, super::clamp_to_size(-8, 11));
        assert_eq!(4, super::clamp_to_size(-7, 11));
        assert_eq!(5, super::clamp_to_size(-6, 11));
        assert_eq!(6, super::clamp_to_size(-5, 11));
        assert_eq!(7, super::clamp_to_size(-4, 11));
        assert_eq!(8, super::clamp_to_size(-3, 11));
        assert_eq!(9, super::clamp_to_size(-2, 11));
        assert_eq!(10, super::clamp_to_size(-1, 11));
        assert_eq!(0, super::clamp_to_size(0, 11));
        assert_eq!(1, super::clamp_to_size(1, 11));
        assert_eq!(2, super::clamp_to_size(2, 11));
        assert_eq!(3, super::clamp_to_size(3, 11));
        assert_eq!(4, super::clamp_to_size(4, 11));
        assert_eq!(5, super::clamp_to_size(5, 11));
        assert_eq!(6, super::clamp_to_size(6, 11));
        assert_eq!(7, super::clamp_to_size(7, 11));
        assert_eq!(8, super::clamp_to_size(8, 11));
        assert_eq!(9, super::clamp_to_size(9, 11));
        assert_eq!(10, super::clamp_to_size(10, 11));
        assert_eq!(0, super::clamp_to_size(11, 11));
        assert_eq!(1, super::clamp_to_size(12, 11));
    }
}
