use parse_int::parse;

static INPUT: &'static str = include_str!("./input");

fn read() -> Vec<i32> {
    INPUT
        .lines()
        .map(|line| parse::<i32>(&line))
        .filter_map(|line| line.ok())
        .collect()
}

fn count((i, n): (usize, &i32), numbers: &Vec<i32>) -> Option<bool> {
    if numbers.get(i - 1)? < n {
        Some(true)
    } else {
        None
    }
}

fn windows((i, n): (usize, &i32), numbers: &Vec<i32>) -> Option<i32> {
    let first = numbers.get(i - 1)?;
    let second = numbers.get(i - 2)?;

    Some(n + first + second)
}

pub fn one() -> usize {
    let numbers = read();

    numbers
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|t| count(t, &numbers))
        .collect::<Vec<bool>>()
        .len()
}

pub fn two() -> usize {
    let numbers = read();

    let windowed: Vec<i32> = numbers
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|t| windows(t, &numbers))
        .collect();

    windowed
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|t| count(t, &windowed))
        .collect::<Vec<bool>>()
        .len()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_one() {
        assert_eq!(1139, super::one())
    }

    #[test]
    fn test_two() {
        assert_eq!(1103, super::two())
    }
}
