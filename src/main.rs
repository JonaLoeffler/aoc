#![feature(iter_map_windows)]
use std::time::{Duration, Instant};

mod aoc2021;
mod aoc2022;
mod aoc2023;

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
    println!(
        "2021-01: {} {}",
        aoc2021::aoc01::one(),
        aoc2021::aoc01::two()
    );

    println!(
        "2021-02: {} {}",
        aoc2021::aoc02::one(),
        aoc2021::aoc02::two()
    );

    println!(
        "2021-03: {} {}",
        aoc2021::aoc03::one(),
        aoc2021::aoc03::two()
    );
    println!(
        "2021-04: {} {}",
        aoc2021::aoc04::one(),
        aoc2021::aoc04::two()
    );
    println!(
        "2022-01: {} {}",
        aoc2022::aoc01::one(),
        aoc2022::aoc01::two()
    );
    println!(
        "2022-02: {} {}",
        aoc2022::aoc02::one(),
        aoc2022::aoc02::two()
    );
    println!(
        "2022-03: {} {}",
        aoc2022::aoc03::one(),
        aoc2022::aoc03::two()
    );
    println!(
        "2022-04: {} {}",
        aoc2022::aoc04::one(),
        aoc2022::aoc04::two()
    );
    println!(
        "2022-05: {} {}",
        aoc2022::aoc05::one(),
        aoc2022::aoc05::two()
    );
    println!(
        "2022-06: {} {}",
        aoc2022::aoc06::one().unwrap_or_default(),
        aoc2022::aoc06::two().unwrap_or_default()
    );
    println!(
        "2022-07: {} {}",
        aoc2022::aoc07::one().unwrap_or_default(),
        aoc2022::aoc07::two().unwrap_or_default()
    );
    println!(
        "2022-08: {} {}",
        aoc2022::aoc08::one().unwrap_or_default(),
        aoc2022::aoc08::two().unwrap_or_default()
    );
    println!(
        "2022-09: {} {}",
        aoc2022::aoc09::one().unwrap_or_default(),
        aoc2022::aoc09::two().unwrap_or_default()
    );

    print("2023-01", run(aoc2023::aoc01::one, aoc2023::aoc01::two));
    print("2023-02", run(aoc2023::aoc02::one, aoc2023::aoc02::two));
    print("2023-03", run(aoc2023::aoc03::one, aoc2023::aoc03::two));
    print("2023-04", run(aoc2023::aoc04::one, aoc2023::aoc04::two));
    print("2023-05", run(aoc2023::aoc05::one, aoc2023::aoc05::two));
    print("2023-06", run(aoc2023::aoc06::one, aoc2023::aoc06::two));
    print("2023-07", run(aoc2023::aoc07::one, aoc2023::aoc07::two));
    print("2023-08", run(aoc2023::aoc08::one, aoc2023::aoc08::two));
    print("2023-09", run(aoc2023::aoc09::one, aoc2023::aoc09::two));
    print("2023-10", run(aoc2023::aoc10::one, aoc2023::aoc10::two));
    print("2023-11", run(aoc2023::aoc11::one, aoc2023::aoc11::two));
    print("2023-12", run(aoc2023::aoc12::one, aoc2023::aoc12::two));
    print("2023-13", run(aoc2023::aoc13::one, aoc2023::aoc13::two));
}
