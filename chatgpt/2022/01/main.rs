#!/usr/bin/env rust-script

use std::fs::File;
use std::io::prelude::*;

fn solve_part_1(input: &str) -> i32 {
    let mut calorie_counts = Vec::new();
    let mut current_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            calorie_counts.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }

    calorie_counts.push(current_calories);

    calorie_counts.sort_by(|a, b| b.cmp(a));

    calorie_counts[0]
}

fn solve_part_2(input: &str) -> i32 {
    let mut calorie_counts = Vec::new();
    let mut current_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            calorie_counts.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }

    calorie_counts.push(current_calories);

    calorie_counts.sort_by(|a, b| b.cmp(a));

    calorie_counts[0] + calorie_counts[1] + calorie_counts[2]
}

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let part_1_solution = solve_part_1(&input);
    println!("Part 1: {}", part_1_solution);

    let part_2_solution = solve_part_2(&input);
    println!("Part 2: {}", part_2_solution);

    Ok(())
}
