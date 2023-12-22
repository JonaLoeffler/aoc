use std::collections::HashMap;

use itertools::Itertools;

static EXAMPLE: &'static str = include_str!("./example");

type Point = (i32, i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    name: char,
    start: Point,
    end: Point,
}

impl Brick {
    fn points(&self) -> Vec<Point> {
        let mut res = Vec::new();

        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                for z in self.start.2..=self.end.2 {
                    res.push((x, y, z));
                }
            }
        }

        res
    }

    fn points_below(&self) -> Vec<Point> {
        self.points()
            .into_iter()
            .map(|(x, y, z)| (x, y, z - 1))
            .collect()
    }

    fn fall(self) -> Self {
        Self {
            name: self.name,
            start: *self.points_below().first().unwrap(),
            end: *self.points_below().last().unwrap(),
        }
    }
}

fn parse(input: &'static str) -> Vec<Brick> {
    input
        .lines()
        .zip('A'..)
        .map(|(l, name)| {
            let mut start = l.split("~").nth(0).unwrap().split(",");
            let mut end = l.split("~").nth(1).unwrap().split(",");

            let start = (
                start.next().unwrap().parse().unwrap(),
                start.next().unwrap().parse().unwrap(),
                start.next().unwrap().parse().unwrap(),
            );
            let end = (
                end.next().unwrap().parse().unwrap(),
                end.next().unwrap().parse().unwrap(),
                end.next().unwrap().parse().unwrap(),
            );

            Brick { name, start, end }
        })
        .collect()
}

fn occupancy(bricks: &Vec<Brick>) -> HashMap<Point, Brick> {
    let mut res = HashMap::new();

    for brick in bricks {
        for point in brick.points() {
            res.insert(point, brick.clone());
        }
    }

    res
}

fn gravity(occupancy: HashMap<Point, Brick>) -> HashMap<Point, Brick> {
    let mut has_changed = true;
    let mut occupancy = occupancy;

    while has_changed {
        has_changed = false;

        let falling: Vec<Brick> = occupancy
            .values()
            .unique()
            .filter_map(|brick| {
                let below_empty = brick
                    .points_below()
                    .iter()
                    .find_map(|p| {
                        if p.2 >= 1 {
                            occupancy.get(p).filter(|below| below != &brick)
                        } else {
                            Some(&Brick {
                                name: '#',
                                start: (-100, -100, 0),
                                end: (100, 100, 0),
                            })
                        }
                    })
                    .is_none();

                if below_empty {
                    Some(brick)
                } else {
                    None
                }
            })
            .cloned()
            .collect();

        for brick in falling {
            has_changed = true;

            for point in brick.points() {
                occupancy.remove(&point);
            }

            let fallen = brick.fall();
            for point in fallen.points() {
                occupancy.insert(point, fallen.clone());
            }
        }
    }

    occupancy
}

pub fn one() -> Option<String> {
    let settled = gravity(occupancy(&parse(EXAMPLE)));

    assert_eq!(settled, gravity(settled.clone()));

    let removable = settled
        .values()
        .unique()
        .filter(|brick| {
            let mut one_removed = settled.clone();
            for point in brick.points() {
                one_removed.remove(&point);
            }

            one_removed == gravity(one_removed.clone())
        })
        .count();

    Some(removable.to_string())
}
pub fn two() -> Option<String> {
    let settled = gravity(occupancy(&parse(EXAMPLE)));

    assert_eq!(settled, gravity(settled.clone()));

    let will_collapse: usize = settled
        .values()
        .unique()
        .map(|brick| {
            let mut one_removed = settled.clone();
            for point in brick.points() {
                one_removed.remove(&point);
            }

            let one_removed_gravity = gravity(one_removed.clone());
            let bricks_b: Vec<&Brick> = one_removed_gravity.values().unique().collect();

            // Count how many bricks have changed after removing one and letting them fall
            one_removed
                .values()
                .unique()
                .filter(|a| !bricks_b.contains(a))
                .count()
        })
        .sum();

    Some(will_collapse.to_string())
}

mod tests {}
