use std::num::ParseIntError;
use std::str::FromStr;

static INPUT: &'static str = include_str!("./input");

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let value = split[1].parse::<i32>()?;

        let result = match split[0] {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            x => panic!("Unknown command {}", &x),
        };

        Ok(result)
    }
}

fn read() -> Vec<Command> {
    INPUT
        .lines()
        .map(Command::from_str)
        .filter_map(|c| c.ok())
        .collect()
}

pub fn one() -> i32 {
    let (position, depth) =
        read()
            .iter()
            .fold((0, 0), |(position, depth), command| match command {
                Command::Forward(value) => (position + value, depth),
                Command::Down(value) => (position, depth + value),
                Command::Up(value) => (position, depth - value),
            });

    position * depth
}

pub fn two() -> i32 {
    let (position, depth, _) = read()
        .iter()
        .fold((0, 0, 0), |(position, depth, aim), command| match command {
            Command::Forward(value) => (position + value, depth + (value * aim), aim),
            Command::Down(value) => (position, depth, aim + value),
            Command::Up(value) => (position, depth, aim - value),
        });

    position * depth
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_one() {
        assert_eq!(1459206, super::one())
    }

    #[test]
    fn test_two() {
        assert_eq!(1320534480, super::two())
    }
}
