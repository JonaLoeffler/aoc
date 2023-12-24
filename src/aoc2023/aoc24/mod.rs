use itertools::Itertools;

static INPUT: &'static str = include_str!("./input");

type Vec3 = [f64; 3];

#[derive(Debug, Clone)]
struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

fn parse() -> Vec<Hailstone> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" @ ");

            let location = split
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()[0..3]
                .try_into()
                .unwrap();
            let velocity = split
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()[0..3]
                .try_into()
                .unwrap();

            Hailstone {
                position: location,
                velocity,
            }
        })
        .collect()
}

fn intersect(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    // let x1 = a.position[0];
    // let y1 = a.position[1];
    // let x2 = a.position[0] + a.velocity[0];
    // let y2 = a.position[1] + a.velocity[1];

    // let x3 = b.position[0];
    // let y3 = b.position[1];
    // let x4 = b.position[0] + b.velocity[0];
    // let y4 = b.position[1] + b.velocity[1];

    // let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4))
    //     / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));
    // let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4))
    //     / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));

    let gradient_a = a.velocity[1] / a.velocity[0];
    let gradient_b = b.velocity[1] / b.velocity[0];

    let y_interval_a = a.position[1] - a.position[0] * gradient_a;
    let y_interval_b = b.position[1] - b.position[0] * gradient_b;

    // ga * x + ya = gb * x + yb
    // ga * x - gb * x = yb - ya
    // x = yb - ya / ga - gb

    let px: f64 = (y_interval_b - y_interval_a) / (gradient_a - gradient_b);
    let py: f64 = gradient_a * px + y_interval_a;

    if px.is_normal() && py.is_normal() {
        // Determine if collision is in the past
        // P = l + n * v;
        // P - l / v
        let nax = (px - a.position[0]) / a.velocity[0];
        let nay = (py - a.position[1]) / a.velocity[1];

        let nbx = (px - b.position[0]) / b.velocity[0];
        let nby = (py - b.position[1]) / b.velocity[1];

        if [nax, nay, nbx, nby].iter().any(|f| f < &0.0) {
            None
        } else {
            Some((px, py))
        }
    } else {
        None
    }
}

fn intersect_inside(a: &Hailstone, b: &Hailstone, low: f64, high: f64) -> bool {
    if let Some((a, b)) = intersect(a, b) {
        a > low && b > low && a < high && b < high
    } else {
        false
    }
}

pub fn one() -> Option<String> {
    Some(
        parse()
            .into_iter()
            .combinations(2)
            // .filter(|h| intersect_inside(&h[0], &h[1], 7.0, 27.0))
            .filter(|h| intersect_inside(&h[0], &h[1], 200000000000000.0, 400000000000000.0))
            .count()
            .to_string(),
    )
}

pub fn two() -> Option<String> {
    None
}

mod tests {}
