static INPUT: &'static str = include_str!("./example");

fn parse() -> Vec<(String, Vec<i32>)> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");
            let fst = split.next().unwrap();

            let snd = split
                .next()
                .unwrap()
                .split(",")
                .filter_map(|n| n.parse::<i32>().ok())
                .collect();

            (fst.to_string(), snd)
        })
        .collect()
}

fn solve_line((springs, groups): (String, Vec<i32>)) -> i32 {
    dbg!(&springs);
    build_recursive(springs.clone())
        .into_iter()
        .filter(|s| fits_constraints((s.to_string(), &groups)))
        .count() as i32
}

fn build_recursive(springs: String) -> Vec<String> {
    if springs.len() == 1 {
        return match springs.as_str() {
            "#" => vec!["#".to_string()],
            "?" => vec!["#".to_string(), ".".to_string()],
            "." => vec![".".to_string()],
            _ => vec![],
        };
    } else {
        let head = springs.chars().nth(0).unwrap().to_string();
        let tail = springs[1..].to_string();

        let res = build_recursive(tail);

        match head.as_str() {
            "#" => res.iter().map(|s| format!("{head}{s}")).collect(),
            "?" => res
                .iter()
                .map(|s| format!("#{s}"))
                .chain(res.iter().map(|s| format!(".{s}")))
                .collect(),
            "." => res.iter().map(|s| format!("{head}{s}")).collect(),
            _ => vec![],
        }
    }
}

fn fits_constraints((springs, groups): (String, &Vec<i32>)) -> bool {
    let res: Vec<bool> = springs
        .split(".")
        .filter(|s| s.contains("#"))
        .enumerate()
        .map(|(i, s)| s.len() == *groups.get(i).unwrap_or(&i32::MAX) as usize)
        .collect();

    res.iter().all(|b| *b) && res.len() == groups.len()
}

pub fn one() -> Option<String> {
    return None;
    #[allow(unreachable_code)]
    Some(parse().into_iter().map(solve_line).sum::<i32>().to_string())
}

pub fn two() -> Option<String> {
    return None;
    #[allow(unreachable_code)]
    Some(
        parse()
            .into_iter()
            .map(|(s, g)| {
                let s = format!("{s}?").repeat(5);

                (s[0..s.len() - 1].to_string(), g.repeat(5))
            })
            .map(solve_line)
            .sum::<i32>()
            .to_string(),
    )
}

mod tests {
    #[test]
    fn test_build_recursive() {
        assert_eq!(
            vec!["###.", "##.."],
            super::build_recursive("##?.".to_string())
        );
    }

    #[test]
    fn test_both() {
        assert_eq!(
            1,
            super::build_recursive("?#?#?#?#?#?#?#?".to_string())
                .into_iter()
                .filter(|s| super::fits_constraints((s.to_string(), &vec![1, 3, 1, 6])))
                .count()
        );
    }

    #[test]
    fn test_fits_constraints() {
        assert!(super::fits_constraints((
            "##...###...##..#".to_string(),
            &vec![2, 3, 2, 1]
        )));

        assert!(!super::fits_constraints((
            "##...###...##..#".to_string(),
            &vec![3, 2, 2, 1]
        )));
    }
}
