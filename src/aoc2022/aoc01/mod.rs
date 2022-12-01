static INPUT: &'static str = include_str!("./input");

fn list() -> Vec<Vec<i32>> {
    INPUT
        .split("\n\n")
        .into_iter()
        .map(|s| s.lines().map(|l| l.parse::<i32>().unwrap()).collect())
        .collect()
}

pub fn part1(input: String) -> Option<String> {
    Some(
        list()
            .iter()
            .map(|e| e.iter().sum::<i32>())
            .max()
            .unwrap()
            .to_string(),
    )
}

pub fn part2(input: String) -> Option<String> {
    let mut list = list();

    list.sort_by(|a, b| {
        let a_sum: i32 = a.iter().sum();
        let b_sum: i32 = b.iter().sum();

        a_sum.partial_cmp(&b_sum).unwrap().reverse()
    });

    Some(
        list.iter()
            .take(3)
            .map(|e: &Vec<i32>| e.iter().sum::<i32>())
            .sum::<i32>()
            .to_string(),
    )
}
