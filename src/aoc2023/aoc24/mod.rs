use std::collections::HashMap;

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
    let x1 = a.position[0];
    let y1 = a.position[1];
    let x2 = a.position[0] + a.velocity[0];
    let y2 = a.position[1] + a.velocity[1];

    let x3 = b.position[0];
    let y3 = b.position[1];
    let x4 = b.position[0] + b.velocity[0];
    let y4 = b.position[1] + b.velocity[1];

    let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4))
        / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));
    let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4))
        / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));

    if px.is_normal() && py.is_normal() {
        // filter if is in the past
        //
        // P = l + n * v;
        // P - l / v
        let nax = (px - a.position[0]) / a.velocity[0];
        let nay = (py - a.position[1]) / a.velocity[1];

        let nbx = (px - b.position[0]) / b.velocity[0];
        let nby = (py - b.position[1]) / b.velocity[1];

        if [nax, nay, nbx, nby].iter().any(|f| f < &0.0) {
            println!("in the past");
            None
        } else {
            println!("intersect at {:?}", (&px, &py));
            Some((px, py))
        }
    } else {
        println!("parallel");
        None
    }
}

fn intersect_inside(a: &Hailstone, b: &Hailstone, low: f64, high: f64) -> bool {
    if let Some((a, b)) = intersect(a, b) {
        let res = a > low && b > low && a < high && b < high;

        println!("is inside: {:#?}", res);

        res
    } else {
        println!("no intersection in the future");
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
