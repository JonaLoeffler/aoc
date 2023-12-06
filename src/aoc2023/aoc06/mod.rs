use std::iter::zip;

static INPUT: &'static str = include_str!("./input");

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn parse(part_two: bool) -> Vec<Race> {
    let input = match part_two {
        true => INPUT.replace(" ", "").to_string(),
        false => INPUT.to_string(),
    };
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

pub fn one() -> Option<String> {
    Some(
        parse(false)
            .into_iter()
            .map(|race| {
                (1..race.time)
                    .map(|time| time * (race.time - time))
                    .filter(|t| t > &race.distance)
                    .count()
            })
            .product::<usize>()
            .to_string(),
    )
}

pub fn two() -> Option<String> {
    Some(
        parse(true)
            .into_iter()
            .map(|race| {
                (1..race.time)
                    .map(|time| time * (race.time - time))
                    .filter(|t| t > &race.distance)
                    .count()
            })
            .product::<usize>()
            .to_string(),
    )
}
