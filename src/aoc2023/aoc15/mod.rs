use std::collections::HashMap;

static INPUT: &'static str = include_str!("./input");

fn parse() -> Vec<String> {
    INPUT
        .replace("\n", "")
        .split(",")
        .map(ToOwned::to_owned)
        .collect()
}

#[derive(Debug)]
enum Operation {
    Equals(usize),
    Dash,
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal: usize,
}

fn parse2() -> Vec<(String, Operation)> {
    INPUT
        .replace("\n", "")
        .split(",")
        .map(|l| match l.contains("=") {
            true => {
                let label = l.split("=").nth(0).unwrap().to_string();
                let focal = l.split("=").nth(1).unwrap().parse::<usize>().unwrap();
                let operation = Operation::Equals(focal);

                (label, operation)
            }
            false => {
                let label = l.split("-").nth(0).unwrap().to_string();
                let operation = Operation::Dash;

                (label, operation)
            }
        })
        .collect()
}

fn hash(s: String) -> usize {
    let mut res = 0;

    for c in s.chars() {
        res = res + c as usize;
        res = res * 17;
        res = res % 256;
    }

    res
}

pub fn one() -> Option<String> {
    Some(parse().into_iter().map(hash).sum::<usize>().to_string())
}

pub fn two() -> Option<String> {
    let p = parse2();

    let mut map: HashMap<usize, Vec<Lens>> = HashMap::new();

    for (label, operation) in p {
        let hash = hash(label.clone());
        let list = map.entry(hash).or_insert(vec![]);

        match operation {
            Operation::Equals(f) => {
                let lens = Lens {
                    label: label.clone(),
                    focal: f,
                };

                if let Some((i, _)) = list.iter().enumerate().find(|(_, l)| l.label == label) {
                    list.remove(i);
                    list.insert(i, lens);
                } else {
                    list.push(lens);
                }
            }
            Operation::Dash => {
                if let Some((i, _)) = list.iter().enumerate().find(|(_, l)| l.label == label) {
                    list.remove(i);
                }
            }
        }
    }

    Some(
        map.into_iter()
            .flat_map(|(b, ls)| {
                ls.into_iter()
                    .enumerate()
                    .map(|(i, l)| (b + 1) * (i + 1) * l.focal)
                    .collect::<Vec<usize>>()
            })
            .sum::<usize>()
            .to_string(),
    )
}

mod tests {
    #[test]
    fn test_hash() {
        assert_eq!(52, super::hash("HASH".to_string()));
    }
}
