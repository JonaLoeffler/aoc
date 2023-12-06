static INPUT: &'static str = include_str!("./input");

pub fn one() -> Option<String> {
    let res: usize = INPUT
        .lines()
        .filter_map(|l| {
            let mut split = l.split(":");

            let game_id = split
                .next()
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let draws: Vec<(usize, String)> = split
                .next()
                .unwrap()
                .split(";")
                .flat_map(|s| s.split(","))
                .filter_map(|s| {
                    let mut split = s.split(" ").filter(|a| !a.is_empty());

                    Some((
                        split.next().unwrap().parse::<usize>().ok()?,
                        split.next().unwrap().to_string(),
                    ))
                })
                .collect();

            for (num, col) in draws {
                let impossible = match col.as_str() {
                    "red" => num > 12,
                    "green" => num > 13,
                    "blue" => num > 14,
                    _default => panic!(),
                };

                if impossible {
                    return None;
                }
            }

            Some(game_id)
        })
        .sum();

    Some(res.to_string())
}

pub fn two() -> Option<String> {
    let res: usize = INPUT
        .lines()
        .filter_map(|l| {
            let mut split = l.split(":");

            let draws: Vec<(usize, String)> = split
                .next()
                .unwrap()
                .split(";")
                .flat_map(|s| s.split(","))
                .filter_map(|s| {
                    let mut split = s.split(" ").filter(|a| !a.is_empty());

                    Some((
                        split.next().unwrap().parse::<usize>().ok()?,
                        split.next().unwrap().to_string(),
                    ))
                })
                .collect();

            let mut least_green = 0;
            let mut least_blue = 0;
            let mut least_red = 0;

            for (num, col) in draws {
                match col.as_str() {
                    "green" if least_green < num => least_green = num,
                    "blue" if least_blue < num => least_blue = num,
                    "red" if least_red < num => least_red = num,
                    &_ => (),
                }
            }

            Some(least_blue * least_green * least_red)
        })
        .sum();

    Some(res.to_string())
}
