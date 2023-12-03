static INPUT: &'static str = include_str!("./input");

fn read() -> Vec<Vec<usize>> {
    INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| s.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

pub fn one() -> Option<String> {
    let input = read();

    let mut visible: Vec<Vec<bool>> = input
        .iter()
        .enumerate()
        .map(|(i, k)| {
            k.iter()
                .enumerate()
                .map(|(j, _)| {
                    i == 0
                        || j == 0
                        || i == input.len() - 1
                        || j == input.first().unwrap().len() - 1
                })
                .collect()
        })
        .collect();

    let rows = input.len();
    let cols = input.first().unwrap().len();

    for i in 0..rows - 1 {
        let mut highest = 0;
        for j in 1..cols - 1 {
            let next = input.get(i).unwrap().get(j).unwrap();
            let prev = input.get(i).unwrap().get(j - 1).unwrap();

            if prev > &highest {
                highest = prev.clone();
            }

            if next > &highest {
                *visible.get_mut(i).unwrap().get_mut(j).unwrap() = true;
            }
        }
        highest = 0;
        for j in (0..cols - 1).rev() {
            let next = input.get(i).unwrap().get(j).unwrap();
            let prev = input.get(i).unwrap().get(j + 1).unwrap();

            if prev > &highest {
                highest = prev.clone();
            }

            if next > &highest {
                *visible.get_mut(i).unwrap().get_mut(j).unwrap() = true;
            }
        }
    }

    for j in 0..cols - 1 {
        let mut highest = 0;
        for i in 1..rows - 1 {
            let next = input.get(i).unwrap().get(j).unwrap();
            let prev = input.get(i - 1).unwrap().get(j).unwrap();

            if prev > &highest {
                highest = prev.clone();
            }

            if next > &highest {
                *visible.get_mut(i).unwrap().get_mut(j).unwrap() = true;
            }
        }
        highest = 0;
        for i in (0..rows - 1).rev() {
            let next = input.get(i).unwrap().get(j).unwrap();
            let prev = input.get(i + 1).unwrap().get(j).unwrap();

            if prev > &highest {
                highest = prev.clone();
            }

            if next > &highest {
                *visible.get_mut(i).unwrap().get_mut(j).unwrap() = true;
            }
        }
    }

    let visible_count = visible.iter().flatten().filter(|b| **b).count();

    Some(visible_count.to_string())
}

pub fn two() -> Option<String> {
    let input = read();

    let mut score: Vec<Vec<i32>> = input
        .iter()
        .map(|k| k.iter().map(|_| 0).collect())
        .collect();

    let rows = input.len();
    let cols = input.first().unwrap().len();

    // for line in &input {
        // println!("{:?}", line);
    // }

    for i in 0..rows {
        for j in 0..cols {
            // println!("looking at point {:?}", (i, j));
            let mut left = 0;
            let mut right = 0;
            let top = 0;
            let bottom = 0;

            // println!("{}", "---left---");
            let highest = input.get(i).unwrap().get(j).unwrap();
            for k in (i + 1)..rows - 1 {
                let next = input.get(k).unwrap().get(j).unwrap();

                // println!("next: {:?}", (k, j, next, highest));

                if next >= highest {
                    // println!("{}", "breaking");
                    break;
                } else {
                    // println!("{}", "incrementing");
                    left += 1;
                }
            }

            // println!("{}", "---right---");
            let highest = input.get(i).unwrap().get(j).unwrap();
            for k in (0..i).rev() {
                let next = input.get(k).unwrap().get(j).unwrap();

                // println!("next: {:?}", (k, j, next, highest));

                if next >= highest {
                    // println!("{}", "breaking");
                    break;
                } else {
                    // println!("{}", "incrementing");
                    right += 1;
                }
            }

            // println!("scores: {:?}", (left, right, bottom, top));
            *score.get_mut(i).unwrap().get_mut(j).unwrap() = left * right * top * bottom;
        }
    }

    // for line in &score {
        // println!("{:?}", line);
    // }

    let highscore = score.iter().flatten().max().unwrap();

    Some(highscore.to_string())
}
