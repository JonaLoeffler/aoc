static INPUT: &'static str = include_str!("./input");

fn parse() -> Vec<Vec<Vec<char>>> {
    INPUT
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn transpose(inner: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..inner[0].len())
        .map(|i| {
            inner
                .iter()
                .map(|inn| inn[i].clone())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn find_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
    let pattern: Vec<String> = pattern
        .iter()
        .map(|l| l.iter().collect::<String>())
        .collect();

    let r = pattern
        .iter()
        .enumerate()
        .map_windows(|[(i1, p1), (i2, p2)]| {
            if p1 == p2 {
                for j in 0..*i1 {
                    let empty = String::new();
                    let lft = pattern.get(j).unwrap_or(&empty);
                    let rgt = pattern.get(i1 + i2 - j).unwrap_or(&empty);

                    if !lft.is_empty() && !rgt.is_empty() && lft != rgt {
                        return None;
                    }
                }

                return Some(i1 + 1);
            }

            None
        })
        .find(|r| r.is_some());

    r?
}

// walk through all possible smudges, remove, check if new different mirror line
// Exclude previous reflection
fn remove_smudge() {
    todo!()
}

pub fn one() -> Option<String> {
    Some(
        parse()
            .iter()
            .map(|pattern| {
                if let Some(r) = find_reflection(pattern) {
                    return r * 100;
                }

                if let Some(r) = find_reflection(&transpose(pattern)) {
                    return r;
                }

                panic!("no reflection!")
            })
            .sum::<usize>()
            .to_string(),
    )
}

pub fn two() -> Option<String> {
    None
}

mod tests {}
