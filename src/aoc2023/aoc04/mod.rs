static INPUT: &'static str = include_str!("./input");

fn parse() -> Vec<(Vec<i32>, Vec<i32>)> {
    INPUT
        .lines()
        .map(|l| -> (Vec<i32>, Vec<i32>) {
            let mut s = l.split(":");
            let mut nums = s.nth(1).unwrap().split("|");

            let n1 = nums.next().unwrap();
            let n2 = nums.next().unwrap();

            (
                n1.split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect(),
                n2.split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn one() -> Option<String> {
    Some(
        parse()
            .into_iter()
            .map(wins_for_card)
            .map(points_for_wins)
            .sum::<i32>()
            .to_string(),
    )
}

fn points_for_wins(w: i32) -> i32 {
    if w == 0 {
        0
    } else {
        2_i32.pow((w - 1).try_into().unwrap())
    }
}

fn wins_for_card((w, a): (Vec<i32>, Vec<i32>)) -> i32 {
    let mut res = 0;

    for n in a {
        if w.contains(&n) {
            res += 1
        }
    }

    res
}

pub fn two() -> Option<String> {
    let cards = parse();

    let mut tmp: Vec<i32> = cards.iter().map(|_| 1).collect();

    for (i, card) in cards.iter().enumerate() {
        let wins = wins_for_card(card.clone());

        let val = tmp.get(i).unwrap().clone();

        for offset in 1..=wins {
            let offset: usize = offset.try_into().unwrap();
            let j: usize = i + offset;

            if let Some(c) = tmp.get_mut(j) {
                *c += val;
            }
        }
    }

    Some(tmp.iter().sum::<i32>().to_string())
}
