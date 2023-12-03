static INPUT: &'static str = include_str!("./input");

pub fn one() -> Option<String> {
    let mut grid: Vec<Vec<char>> = INPUT
        .lines()
        .map(|l| format!(".{}.", l.to_string()).chars().collect())
        .collect();

    grid.insert(0, (0..grid.get(0).unwrap().len()).map(|_| '.').collect());
    grid.push((0..grid.get(0).unwrap().len()).map(|_| '.').collect());

    let mut res = 0;

    for row in 0..grid.len() {
        for col in 0..grid.get(row).unwrap().len() {
            let c = grid.get(row).unwrap().get(col).unwrap();

            let mut numbers: Vec<i32> = vec![];

            if !c.is_numeric() && c != &'.' {
                numbers.extend(complete_row(&mut grid, row - 1, col - 1, col + 1).iter());
                numbers.extend(complete_row(&mut grid, row, col - 1, col + 1).iter());
                numbers.extend(complete_row(&mut grid, row + 1, col - 1, col + 1).iter());
            }

            res += numbers.iter().sum::<i32>();
        }
    }

    Some(res.to_string())
}

fn complete_row(grid: &mut Vec<Vec<char>>, row: usize, start: usize, end: usize) -> Vec<i32> {
    let mut start = start;
    let mut end = end;

    let row = grid.get_mut(row).unwrap();

    if row.get(start).unwrap_or(&'x').is_numeric() {
        start -= 1;

        if row.get(start).unwrap_or(&'x').is_numeric() {
            start -= 1;
        }
    }
    if row.get(end).unwrap_or(&'x').is_numeric() {
        end += 1;
        if row.get(end).unwrap_or(&'x').is_numeric() {
            end += 1;
        }
    }

    let slice = row[start..=end].iter().collect::<String>();

    row.splice(
        start..=end,
        slice.chars().map(|c| match c.is_numeric() {
            true => '.',
            false => c,
        }),
    );

    slice
        .split(|s: char| !s.is_numeric())
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}

pub fn two() -> Option<String> {
    let mut grid: Vec<Vec<char>> = INPUT
        .lines()
        .map(|l| format!(".{}.", l.to_string()).chars().collect())
        .collect();

    grid.insert(0, (0..grid.get(0).unwrap().len()).map(|_| '.').collect());
    grid.push((0..grid.get(0).unwrap().len()).map(|_| '.').collect());

    let mut res = 0;

    for row in 0..grid.len() {
        for col in 0..grid.get(row).unwrap().len() {
            let c = grid.get(row).unwrap().get(col).unwrap();

            let mut numbers: Vec<i32> = vec![];

            if !c.is_numeric() && c == &'*' {
                numbers.extend(complete_row(&mut grid, row - 1, col - 1, col + 1).iter());
                numbers.extend(complete_row(&mut grid, row, col - 1, col + 1).iter());
                numbers.extend(complete_row(&mut grid, row + 1, col - 1, col + 1).iter());
            }

            if numbers.len() == 2 {
                res += numbers.iter().product::<i32>();
            }
        }
    }

    Some(res.to_string())
}
