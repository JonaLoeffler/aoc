static INPUT: &'static str = include_str!("./input");

fn list() -> Vec<(&'static str, &'static str)> {
    INPUT
        .lines()
        .map(|l| {
            let mut items = l.split(" ");
            (items.next().unwrap(), items.next().unwrap())
        })
        .collect()
}

fn score(played: (&str, &str)) -> i32 {
    let own = match played.1 {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!(),
    };

    // Rock A X
    // Paper B Y
    // Scissors C Z
    //
    // X Lose
    // Y Draw
    // Z Win

    let other: i32 = match played {
        ("A", "B") => 6,
        ("B", "C") => 6,
        ("C", "A") => 6,
        ("A", "A") => 3,
        ("B", "B") => 3,
        ("C", "C") => 3,
        _ => 0,
    };

    other + own
}

pub fn one() -> i32 {
    list()
        .into_iter()
        .map(|(a, b)| {
            (
                a,
                match b {
                    "X" => "A",
                    "Y" => "B",
                    "Z" => "C",
                    _ => panic!(),
                },
            )
        })
        .map(score)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

pub fn two() -> i32 {
    list()
        .into_iter()
        .map(|(a, b)| {
            (
                a,
                match b {
                    "X" => match a {
                        "A" => "C",
                        "B" => "A",
                        "C" => "B",
                        _ => panic!(),
                    },
                    "Y" => match a {
                        "A" => "A",
                        "B" => "B",
                        "C" => "C",
                        _ => panic!(),
                    },
                    "Z" => match a {
                        "A" => "B",
                        "B" => "C",
                        "C" => "A",
                        _ => panic!(),
                    },
                    _ => panic!(),
                },
            )
        })
        .map(score)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}
