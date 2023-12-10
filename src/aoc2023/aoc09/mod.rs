static INPUT: &'static str = include_str!("./input");

pub fn one() -> Option<String> {
    Some(
        INPUT
            .lines()
            .map(|l| l.split(" ").filter_map(|l| l.parse::<i32>().ok()).collect())
            .map(line)
            .sum::<i32>()
            .to_string(),
    )
}

fn line(seq: Vec<i32>) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![seq];

    while sequences.last().unwrap().iter().any(|n| n != &0) {
        let new = sequences
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w.get(1).unwrap() - w.get(0).unwrap())
            .collect();

        sequences.push(new);
    }

    sequences.last_mut().unwrap().push(0);

    for i in (0..sequences.len()).rev().skip(1) {
        let next = sequences.get(i).unwrap().last().unwrap()
            + sequences.get(i + 1).unwrap().last().unwrap();

        sequences.get_mut(i).unwrap().push(next);
    }

    *sequences.first().unwrap().last().unwrap()
}

pub fn two() -> Option<String> {
    Some(
        INPUT
            .lines()
            .map(|l| {
                l.split(" ")
                    .filter_map(|l| l.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .map(|mut l| {
                l.reverse();
                l
            })
            .map(line)
            .sum::<i32>()
            .to_string(),
    )
}

mod tests {}
