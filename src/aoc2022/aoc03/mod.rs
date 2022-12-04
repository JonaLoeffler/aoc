static INPUT: &'static str = include_str!("./input");

fn priorities(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - 96
    } else {
        c as i32 - 64 + 26
    }
}

pub fn one() -> i32 {
    INPUT
        .lines()
        .map(|l| {
            let mut fst = l.to_string();
            let snd = fst.split_off(l.len() / 2);
            (fst, snd)
        })
        .map(|(fst, snd)| fst.chars().filter(|c| snd.contains(*c)).next().unwrap())
        .map(priorities)
        .sum()
}
pub fn two() -> i32 {
    let lines = INPUT
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    lines
        .chunks(3)
        .map(|w| {
            let fst = w.get(0).unwrap();

            fst.chars()
                .filter(|c| w.get(1).unwrap().contains(*c))
                .filter(|c| w.get(2).unwrap().contains(*c))
                .next()
                .unwrap()
        })
        .map(priorities)
        .sum()
}
