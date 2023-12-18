type Color = String;

static INPUT: &'static str = include_str!("./input");

fn parse2() -> Vec<(char, i64, Color)> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            let col = split.nth(2).unwrap().replace(")", "").replace("(", "");

            let dir = match col.chars().last().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                s => panic!("unexpected last {s}"),
            };

            let len = i64::from_str_radix(&col.chars().skip(1).take(5).collect::<String>(), 16)
                .expect("parse");

            (dir, len, col)
        })
        .collect()
}

fn parse1() -> Vec<(char, i64, Color)> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            let dir = split.next().unwrap().chars().nth(0).unwrap();

            let len = split.next().unwrap().parse::<i64>().unwrap();

            let col = split.next().unwrap().replace(")", "").replace("(", "");

            (dir, len, col)
        })
        .collect()
}

fn run(instructions: &Vec<(char, i64, String)>) -> i64 {
    let mut area = 0;
    let mut edges = 0;

    let mut x1 = 0;
    let mut y1 = 0;

    for (dir, len, _) in instructions.iter() {
        let (x2, y2) = match dir {
            'R' => (x1 + len, y1),
            'L' => (x1 - len, y1),
            'U' => (x1, y1 + len),
            'D' => (x1, y1 - len),
            _ => panic!(),
        };

        area += (x1 * y2) - (x2 * y1);
        edges += len;

        (x1, y1) = (x2, y2);
    }

    (area / 2).abs() + (edges / 2) + 1
}

pub fn one() -> Option<String> {
    Some(run(&parse1()).to_string())
}

pub fn two() -> Option<String> {
    Some(run(&parse2()).to_string())
}

mod tests {}
