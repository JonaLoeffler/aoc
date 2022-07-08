use parse_int::parse;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn read() -> io::Result<Vec<i32>> {
    let file = File::open("./src/aoc2021/aoc01/input")?;
    let reader = BufReader::new(file);

    let result: Vec<i32> = reader
        .lines()
        .map(|line| parse::<i32>(&line.unwrap_or_default()))
        .filter_map(|line| line.ok())
        .collect();

    Ok(result)
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
    let numbers = read().unwrap_or_default();

    numbers
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|t| count(t, &numbers))
        .collect::<Vec<bool>>()
        .len()
}

pub fn two() -> usize {
    let numbers = read().unwrap_or_default();

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
