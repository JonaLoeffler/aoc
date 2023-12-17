#![feature(iter_map_windows)]
use chrono::Datelike;
use clap::Parser;
use std::time::{Duration, Instant};

mod aoc2021;
mod aoc2022;
mod aoc2023;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What year to execute
    #[arg(short, long)]
    year: Option<i32>,

    /// What day to execute
    #[arg(short, long)]
    day: Option<u32>,
}

type DayResult = (Duration, String, String);

fn run(one: fn() -> Option<String>, two: fn() -> Option<String>) -> DayResult {
    let now = Instant::now();

    let res = (one(), two());

    (
        now.elapsed(),
        res.0.unwrap_or_default(),
        res.1.unwrap_or_default(),
    )
}

fn print(day: &str, res: DayResult) -> () {
    let (d, one, two) = res;

    println!("{day}: {0:04}ms {one} {two}", d.as_millis());
}

fn main() {
    let args = Args::parse();
    let date = chrono::Local::now();

    match args.year.unwrap_or(date.year()) {
        2021 => match args.day.unwrap_or(date.day()) {
            1 => println!(
                "2021-01: {} {}",
                aoc2021::aoc01::one(),
                aoc2021::aoc01::two()
            ),
            2 => println!(
                "2021-02: {} {}",
                aoc2021::aoc02::one(),
                aoc2021::aoc02::two()
            ),
            3 => println!(
                "2021-03: {} {}",
                aoc2021::aoc03::one(),
                aoc2021::aoc03::two()
            ),
            4 => println!(
                "2021-04: {} {}",
                aoc2021::aoc04::one(),
                aoc2021::aoc04::two()
            ),
            d => println!("Cannot execute day {}", d),
        },
        2022 => match args.day.unwrap_or(date.day()) {
            1 => println!(
                "2022-01: {} {}",
                aoc2022::aoc01::one(),
                aoc2022::aoc01::two()
            ),
            2 => println!(
                "2022-02: {} {}",
                aoc2022::aoc02::one(),
                aoc2022::aoc02::two()
            ),
            3 => println!(
                "2022-03: {} {}",
                aoc2022::aoc03::one(),
                aoc2022::aoc03::two()
            ),
            4 => println!(
                "2022-04: {} {}",
                aoc2022::aoc04::one(),
                aoc2022::aoc04::two()
            ),
            5 => println!(
                "2022-05: {} {}",
                aoc2022::aoc05::one(),
                aoc2022::aoc05::two()
            ),
            6 => println!(
                "2022-06: {} {}",
                aoc2022::aoc06::one().unwrap_or_default(),
                aoc2022::aoc06::two().unwrap_or_default()
            ),
            7 => println!(
                "2022-07: {} {}",
                aoc2022::aoc07::one().unwrap_or_default(),
                aoc2022::aoc07::two().unwrap_or_default()
            ),
            8 => println!(
                "2022-08: {} {}",
                aoc2022::aoc08::one().unwrap_or_default(),
                aoc2022::aoc08::two().unwrap_or_default()
            ),
            9 => println!(
                "2022-09: {} {}",
                aoc2022::aoc09::one().unwrap_or_default(),
                aoc2022::aoc09::two().unwrap_or_default()
            ),
            d => println!("Cannot execute day {}", d),
        },
        2023 => match args.day.unwrap_or(date.day()) {
            1 => print("2023-01", run(aoc2023::aoc01::one, aoc2023::aoc01::two)),
            2 => print("2023-02", run(aoc2023::aoc02::one, aoc2023::aoc02::two)),
            3 => print("2023-03", run(aoc2023::aoc03::one, aoc2023::aoc03::two)),
            4 => print("2023-04", run(aoc2023::aoc04::one, aoc2023::aoc04::two)),
            5 => print("2023-05", run(aoc2023::aoc05::one, aoc2023::aoc05::two)),
            6 => print("2023-06", run(aoc2023::aoc06::one, aoc2023::aoc06::two)),
            7 => print("2023-07", run(aoc2023::aoc07::one, aoc2023::aoc07::two)),
            8 => print("2023-08", run(aoc2023::aoc08::one, aoc2023::aoc08::two)),
            9 => print("2023-09", run(aoc2023::aoc09::one, aoc2023::aoc09::two)),
            10 => print("2023-10", run(aoc2023::aoc10::one, aoc2023::aoc10::two)),
            11 => print("2023-11", run(aoc2023::aoc11::one, aoc2023::aoc11::two)),
            12 => print("2023-12", run(aoc2023::aoc12::one, aoc2023::aoc12::two)),
            13 => print("2023-13", run(aoc2023::aoc13::one, aoc2023::aoc13::two)),
            14 => print("2023-14", run(aoc2023::aoc14::one, aoc2023::aoc14::two)),
            15 => print("2023-15", run(aoc2023::aoc15::one, aoc2023::aoc15::two)),
            16 => print("2023-16", run(aoc2023::aoc16::one, aoc2023::aoc16::two)),
            17 => print("2023-17", run(aoc2023::aoc17::one, aoc2023::aoc17::two)),
            d => println!("Cannot execute day {}", d),
        },
        y => println!("Cannot execute year {}", y),
    };
}
