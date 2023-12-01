use std::collections::HashMap;

static INPUT: &'static str = include_str!("./input");

pub fn one() -> Option<String> {
    Some(
        INPUT
            .lines()
            .into_iter()
            .map(parse_numbers)
            .map(concat_first_and_last)
            .sum::<i32>()
            .to_string(),
    )
}

fn parse_numbers<S: AsRef<str>>(s: S) -> Vec<i32> {
    s.as_ref()
        .split("")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn concat_first_and_last(v: Vec<i32>) -> i32 {
    (&format!("{}{}", v.first().unwrap(), v.last().unwrap()))
        .parse::<i32>()
        .unwrap()
}

fn replace(str: &str) -> String {
    let mut str: String = str.to_owned();

    let replace: HashMap<&str, &str> = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]
    .into_iter()
    .collect();

    let mut lo = 0;
    let mut hi = 1;

    while hi <= str.len() {
        let part = str[lo..hi].to_string();

        for key in replace.keys() {
            if part.contains(key) {
                if let Some(val) = replace.get(key) {
                    str.replace_range(lo..hi, &part.replace(key, val));
                    lo += 2;
                    hi = lo + 1;
                    break;
                }
            }
        }

        hi += 1;

        if (lo..hi).len() > 5 {
            lo += 1;
        }
    }

    str.to_owned()
}

fn replace2(str: &str) -> String {
    str.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

pub fn two() -> Option<String> {
    Some(
        INPUT
            .lines()
            .into_iter()
            .map(replace2)
            .map(parse_numbers)
            .map(concat_first_and_last)
            .sum::<i32>()
            .to_string(),
    )
}

mod tests {}
