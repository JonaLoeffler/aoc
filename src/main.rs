mod aoc2021;
mod aoc2022;

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
}
