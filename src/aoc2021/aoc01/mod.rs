use parse_int::parse;

static INPUT: &'static str = include_str!("./input");

fn read() -> Vec<i32> {
    INPUT
        .lines()
        .map(|line| parse::<i32>(&line))
        .filter_map(|line| line.ok())
        .collect()
}

pub fn part1(input: String) -> Option<String> {
    Some(
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
            .to_string(),
    )
}

pub fn part2(input: String) -> Option<String> {
    Some(
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
            .to_string(),
    )
}
