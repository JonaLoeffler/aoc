use std::ops::Range;

static INPUT: &'static str = include_str!("./input");

fn parse_two() -> (Vec<Range<i64>>, Vec<Vec<(i64, i64, i64)>>) {
    let seeds = INPUT
        .lines()
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .expect("tobethere")
        .split(" ")
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|s| {
            let start = s.get(0).unwrap().parse::<i64>().unwrap();
            let length = s.get(1).unwrap().parse::<i64>().unwrap();

            start..(start + length)
        })
        .collect::<Vec<Range<_>>>();

    let maps = INPUT
        .split("\n\n")
        .skip(1)
        .map(|b| {
            let lines = b.lines().skip(1);

            let mut map = Vec::new();

            for line in lines {
                let mut parts = line.split(" ");

                let (fst, snd, thd) = (
                    parts.next().unwrap().parse::<i64>().unwrap(),
                    parts.next().unwrap().parse::<i64>().unwrap(),
                    parts.next().unwrap().parse::<i64>().unwrap(),
                );

                map.push((fst, snd, thd));
            }

            map
        })
        .collect();

    (seeds, maps)
}
fn parse_one() -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let seeds = INPUT
        .lines()
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .expect("tobethere")
        .split(" ")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let maps = INPUT
        .split("\n\n")
        .skip(1)
        .map(|b| {
            let lines = b.lines().skip(1);

            let mut map = Vec::new();

            for line in lines {
                let mut parts = line.split(" ");

                let (fst, snd, thd) = (
                    parts.next().unwrap().parse::<i64>().unwrap(),
                    parts.next().unwrap().parse::<i64>().unwrap(),
                    parts.next().unwrap().parse::<i64>().unwrap(),
                );

                map.push((fst, snd, thd));
            }

            map
        })
        .collect();

    (seeds, maps)
}

pub fn one() -> Option<String> {
    let (seeds, maps) = parse_one();

    let mut locations = Vec::new();

    for seed in seeds {
        let mut cur = seed.clone();
        for map in &maps {
            'a: for (dest, source, length) in map {
                if cur >= *source && cur < (source + length) {
                    cur = dest + (cur - source);
                    break 'a;
                }
            }
        }

        locations.push(cur);
    }

    Some(locations.iter().min()?.to_string())
}

#[allow(unreachable_code)]
pub fn two() -> Option<String> {
    return None;
    let (seeds, maps) = parse_two();

    let mut locations = Vec::new();

    for range in seeds {
        for seed in range {
            let mut cur = seed.clone();
            for map in &maps {
                'a: for (dest, source, length) in map {
                    if cur >= *source && cur < (source + length) {
                        cur = dest + (cur - source);
                        break 'a;
                    }
                }
            }

            locations.push(cur);
        }
    }

    Some(locations.iter().min()?.to_string())
}
