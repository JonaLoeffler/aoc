use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!("./input");

#[derive(Eq, PartialEq, Hash, Ord, Clone, Copy)]
struct Hand<T: Card>([T; 5]);

#[derive(Eq, PartialEq, Hash, Ord)]
struct Play<T>
where
    T: Card,
    Play<T>: PartialOrd,
{
    hand: Hand<T>,
    hand_with_joker: Hand<T>,
    bid: i32,
}

impl<T: Card> std::fmt::Debug for Hand<T> {
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

impl<T: Card> std::fmt::Debug for Play<T>
where
    Play<T>: PartialOrd,
{
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
enum Card1 {
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
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
enum Card2 {
    A,
    K,
    Q,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    J,
}

trait Card:
    From<char> + Ord + PartialOrd + std::fmt::Debug + std::hash::Hash + PartialEq + Eq + Clone + Copy
{
    fn joker() -> Self;
    fn iterator<'a>() -> std::iter::Copied<std::slice::Iter<'a, Self>>;
}

impl Card for Card1 {
    fn joker() -> Self {
        Self::J
    }

    fn iterator<'a>() -> std::iter::Copied<std::slice::Iter<'a, Self>> {
        [
            Self::A,
            Self::K,
            Self::Q,
            Self::J,
            Self::T,
            Self::C9,
            Self::C8,
            Self::C7,
            Self::C6,
            Self::C5,
            Self::C4,
            Self::C3,
            Self::C2,
        ]
        .iter()
        .copied()
    }
}

impl Card for Card2 {
    fn joker() -> Self {
        Self::J
    }

    fn iterator<'a>() -> std::iter::Copied<std::slice::Iter<'a, Self>> {
        [
            Self::A,
            Self::K,
            Self::Q,
            Self::J,
            Self::T,
            Self::C9,
            Self::C8,
            Self::C7,
            Self::C6,
            Self::C5,
            Self::C4,
            Self::C3,
            Self::C2,
        ]
        .iter()
        .copied()
    }
}

impl From<char> for Card1 {
    fn from(value: char) -> Self {
        match value {
            'A' => Card1::A,
            'K' => Card1::K,
            'Q' => Card1::Q,
            'J' => Card1::J,
            'T' => Card1::T,
            '9' => Card1::C9,
            '8' => Card1::C8,
            '7' => Card1::C7,
            '6' => Card1::C6,
            '5' => Card1::C5,
            '4' => Card1::C4,
            '3' => Card1::C3,
            '2' => Card1::C2,
            _default => panic!(),
        }
    }
}
impl From<char> for Card2 {
    fn from(value: char) -> Self {
        match value {
            'A' => Card2::A,
            'K' => Card2::K,
            'Q' => Card2::Q,
            'J' => Card2::J,
            'T' => Card2::T,
            '9' => Card2::C9,
            '8' => Card2::C8,
            '7' => Card2::C7,
            '6' => Card2::C6,
            '5' => Card2::C5,
            '4' => Card2::C4,
            '3' => Card2::C3,
            '2' => Card2::C2,
            _default => panic!(),
        }
    }
}

impl<T: Card> Hand<T> {
    fn r#type(&self) -> Type {
        let mut unique: HashMap<T, i32> = HashMap::new();

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

impl<T: Card> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for i in 0..5 {
            if self.0[i] != other.0[i] {
                return self.0[i].partial_cmp(&other.0[i]);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Play<Card1> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let t1 = self.hand.r#type();
        let t2 = other.hand.r#type();

        if t1 != t2 {
            t1.partial_cmp(&t2)
        } else {
            self.hand.partial_cmp(&other.hand)
        }
    }
}

impl PartialOrd for Play<Card2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let t1 = self.hand_with_joker.r#type();
        let t2 = other.hand_with_joker.r#type();

        if t1 != t2 {
            t1.partial_cmp(&t2)
        } else {
            self.hand.partial_cmp(&other.hand)
        }
    }
}

#[allow(unused)]
fn apply_joker<T>(hand: &Hand<T>) -> Hand<T>
where
    T: Card,
    Play<T>: PartialOrd,
{
    let mut options: HashSet<Play<T>> = HashSet::new();

    let joker_indices = hand
        .0
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&T::joker())
        .collect::<Vec<(usize, &T)>>();

    let replacements = (0..joker_indices.len())
        .map(|_| T::iterator())
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

    let mut options: Vec<Play<T>> = options
        .into_iter()
        .filter(|h| !h.hand.0.iter().any(|c| c == &T::joker()))
        .collect();

    options.iter().max().map(|p| p.hand).unwrap_or(*hand)
}

fn parse1() -> Vec<Play<Card1>> {
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
                bid: bid.parse::<i32>().unwrap(),
            }
        })
        .collect()
}

fn parse2() -> Vec<Play<Card2>> {
    parse1().into_iter().map(|p| p.into()).collect()
}

impl From<Card1> for Card2 {
    fn from(value: Card1) -> Self {
        match value {
            Card1::A => Card2::A,
            Card1::K => Card2::K,
            Card1::Q => Card2::Q,
            Card1::J => Card2::J,
            Card1::T => Card2::T,
            Card1::C9 => Card2::C9,
            Card1::C8 => Card2::C8,
            Card1::C7 => Card2::C7,
            Card1::C6 => Card2::C6,
            Card1::C5 => Card2::C5,
            Card1::C4 => Card2::C4,
            Card1::C3 => Card2::C3,
            Card1::C2 => Card2::C2,
        }
    }
}

impl From<Play<Card1>> for Play<Card2> {
    fn from(value: Play<Card1>) -> Self {
        let new: Hand<Card2> = Hand(value.hand.0.map(|c| c.into()));

        Play {
            hand: new,
            hand_with_joker: apply_joker(&new),
            bid: value.bid,
        }
    }
}

pub fn one() -> Option<String> {
    let mut inputs: Vec<Play<Card1>> = parse1();

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
    let mut inputs: Vec<Play<Card2>> = parse2();

    inputs.sort();

    let res: i32 = inputs
        .iter()
        .rev()
        .enumerate()
        .map(|(r, p)| (r + 1) as i32 * p.bid)
        .sum();

    Some(res.to_string())
}
