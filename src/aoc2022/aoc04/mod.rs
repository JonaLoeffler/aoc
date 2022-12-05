static INPUT: &'static str = include_str!("./input");

fn input() -> Vec<(i32, i32, i32, i32)> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(",");
            let fst = split.next().unwrap();
            let snd = split.next().unwrap();

            (
                fst.split("-").nth(0).unwrap().parse::<i32>().unwrap(),
                fst.split("-").nth(1).unwrap().parse::<i32>().unwrap(),
                snd.split("-").nth(0).unwrap().parse::<i32>().unwrap(),
                snd.split("-").nth(1).unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

pub fn one() -> i32 {
    input()
        .into_iter()
        .filter(|(a, b, c, d)| (a <= c && b >= d) || (c <= a && d >= b))
        .count() as i32
}

pub fn two() -> i32 {
    input()
        .into_iter()
        .filter(|(a, b, c, d)| {
            (a.clone()..=b.clone())
                .filter(|x| (c..=d).contains(&x))
                .next()
                .is_some()
        })
        .count() as i32
}
