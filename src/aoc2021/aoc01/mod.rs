use parse_int::parse;

static INPUT: &'static str = include_str!("./input");

fn read() -> Vec<i32> {
    INPUT
        .lines()
        .map(|line| parse::<i32>(&line))
        .filter_map(|line| line.ok())
        .collect()
}

pub fn one() -> usize {
    read()
        .windows(2)
        .filter_map(|window| {
            if window.get(0)? < window.get(1)? {
                Some(true)
            } else {
                None
            }
        })
        .collect::<Vec<bool>>()
        .len()
}

pub fn two() -> usize {
    read()
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter_map(|window| {
            if window.get(0)? < window.get(1)? {
                Some(true)
            } else {
                None
            }
        })
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
