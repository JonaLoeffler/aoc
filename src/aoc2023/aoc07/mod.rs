use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!("./input");

#[derive(Eq, PartialEq, Hash, Ord, Clone, Copy)]
struct Hand([Card; 5]);

#[derive(Eq, PartialEq, Hash, Ord)]
struct Play {
    hand: Hand,
    hand_with_joker: Hand,
    bid: i32,
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .map(|h| format!("{:?}", h).replace("C", ""))
                .iter()
                .join(""),
        )
    }
}

impl std::fmt::Debug for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{:?}; ({:?}); {}",
            self.hand, self.hand_with_joker, self.bid
        ))
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    // J,
}

impl Card {
    pub fn iterator<'a>() -> std::iter::Copied<std::slice::Iter<'a, Card>> {
        [
            Card::A,
            Card::K,
            Card::Q,
            Card::J,
            Card::T,
            Card::C9,
            Card::C8,
            Card::C7,
            Card::C6,
            Card::C5,
            Card::C4,
            Card::C3,
            Card::C2,
        ]
        .iter()
        .copied()
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::C9,
            '8' => Card::C8,
            '7' => Card::C7,
            '6' => Card::C6,
            '5' => Card::C5,
            '4' => Card::C4,
            '3' => Card::C3,
            '2' => Card::C2,
            _default => panic!(),
        }
    }
}

impl Hand {
    fn r#type(&self) -> Type {
        let mut unique: HashMap<Card, i32> = HashMap::new();

        for i in self.0 {
            if let Some(entry) = unique.get_mut(&i) {
                *entry += 1;
            } else {
                unique.insert(i, 1);
            }
        }

        let max = unique.values().max().unwrap();

        match unique.len() {
            1 => Type::FiveOfAKind,
            2 if max == &4 => Type::FourOfAKind,
            2 if max == &3 => Type::FullHouse,
            3 if max == &3 => Type::ThreeOfAKind,
            3 if max == &2 => Type::TwoPair,
            4 => Type::OnePair,
            5 => Type::HighCard,
            _default => panic!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for i in 0..5 {
            if self.0[i] != other.0[i] {
                return self.0[i].partial_cmp(&other.0[i]);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // let t1 = self.hand_with_joker.r#type();
        // let t2 = other.hand_with_joker.r#type();
        let t1 = self.hand.r#type();
        let t2 = other.hand.r#type();

        if t1 != t2 {
            t1.partial_cmp(&t2)
        } else {
            self.hand.partial_cmp(&other.hand)
        }
    }
}

fn parse() -> Vec<Play> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            let hand = split.next().unwrap();
            let bid = split.next().unwrap();

            let hand = [
                hand.chars().nth(0).unwrap().into(),
                hand.chars().nth(1).unwrap().into(),
                hand.chars().nth(2).unwrap().into(),
                hand.chars().nth(3).unwrap().into(),
                hand.chars().nth(4).unwrap().into(),
            ];

            Play {
                hand: Hand(hand),
                hand_with_joker: Hand(hand),
                // hand_with_joker: apply_joker(&Hand(hand)),
                bid: bid.parse::<i32>().unwrap(),
            }
        })
        .collect()
}

#[allow(unused)]
fn apply_joker(hand: &Hand) -> Hand {
    let mut options: HashSet<Play> = HashSet::new();

    let joker_indices = hand
        .0
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&Card::J)
        .collect::<Vec<(usize, &Card)>>();

    let replacements = (0..joker_indices.len())
        .map(|_| Card::iterator())
        .multi_cartesian_product();

    for replacement in replacements {
        let mut new = hand.0.clone();

        for (i, (j, _)) in joker_indices.iter().enumerate() {
            new[*j] = replacement.get(i).unwrap().clone();
        }

        options.insert(Play {
            hand: Hand(new),
            hand_with_joker: Hand(new),
            bid: 0,
        });
    }

    let mut options: Vec<Play> = options
        .into_iter()
        .filter(|h| !h.hand.0.iter().any(|c| c == &Card::J))
        .collect();

    options.sort();

    options.first().map(|p| p.hand).unwrap_or(*hand)
}

pub fn one() -> Option<String> {
    let mut inputs = parse();

    inputs.sort();

    let res: i32 = inputs
        .iter()
        .rev()
        .enumerate()
        .map(|(r, p)| (r + 1) as i32 * p.bid)
        .sum();

    Some(res.to_string())
}

pub fn two() -> Option<String> {
    // Move J to bottom
    // Switch implementation of PartialOrd for Play
    // Activate joker apply
    None
}
