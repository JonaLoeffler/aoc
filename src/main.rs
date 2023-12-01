mod aoc2021;
mod aoc2022;
mod aoc2023;

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
        aoc2022::aoc06::one().unwrap(),
        aoc2022::aoc06::two().unwrap()
    );
    // println!(
    //     "2022-07: {} {}",
    //     aoc2022::aoc07::one().unwrap(),
    //     aoc2022::aoc07::two().unwrap()
    // );
    // println!(
    //     "2022-08: {} {}",
    //     aoc2022::aoc08::one().unwrap(),
    //     aoc2022::aoc08::two().unwrap()
    // );
    println!(
        "2023-01: {} {}",
        aoc2023::aoc01::one().unwrap(),
        aoc2023::aoc01::two().unwrap()
    );
}
