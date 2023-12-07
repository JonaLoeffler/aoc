use std::collections::HashMap;

static INPUT: &'static str = include_str!("./input");

#[derive(Eq, PartialEq, Ord, Debug)]
struct Play {
    hand: (Card, Card, Card, Card, Card),
    bid: i32,
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

impl Play {
    fn r#type(&self) -> Type {
        let slice = [
            self.hand.0,
            self.hand.1,
            self.hand.2,
            self.hand.3,
            self.hand.4,
        ];

        let mut unique: HashMap<Card, i32> = HashMap::new();
        for i in slice {
            if let Some(entry) = unique.get_mut(&i) {
                *entry += 1;
            } else {
                unique.insert(i, 1);
            }
        }

        let max = unique.values().max().unwrap();

        match unique.len() {
            5 => Type::HighCard,
            4 => Type::OnePair,
            3 if max == &2 => Type::TwoPair,
            3 if max == &3 => Type::ThreeOfAKind,
            2 if max == &3 => Type::FullHouse,
            2 if max == &4 => Type::FourOfAKind,
            1 => Type::FiveOfAKind,
            _default => panic!(),
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let t1 = self.r#type();
        let t2 = other.r#type();

        if t1 != t2 {
            return t1.partial_cmp(&t2);
        }

        if self.hand.0 != other.hand.0 {
            return self.hand.0.partial_cmp(&other.hand.0);
        } else if self.hand.1 != other.hand.1 {
            return self.hand.1.partial_cmp(&other.hand.1);
        } else if self.hand.2 != other.hand.2 {
            return self.hand.2.partial_cmp(&other.hand.2);
        } else if self.hand.3 != other.hand.3 {
            return self.hand.3.partial_cmp(&other.hand.3);
        } else if self.hand.4 != other.hand.4 {
            return self.hand.4.partial_cmp(&other.hand.4);
        }

        return Some(std::cmp::Ordering::Equal);
    }
}

fn parse() -> Vec<Play> {
    INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" ");

            let hand = split.next().unwrap();
            let bid = split.next().unwrap();

            Play {
                hand: (
                    hand.chars().nth(0).unwrap().into(),
                    hand.chars().nth(1).unwrap().into(),
                    hand.chars().nth(2).unwrap().into(),
                    hand.chars().nth(3).unwrap().into(),
                    hand.chars().nth(4).unwrap().into(),
                ),

                bid: bid.parse::<i32>().unwrap(),
            }
        })
        .collect()
}

pub fn one() -> Option<String> {
    let mut inputs = parse();

    inputs.sort();

    let res: i32 = inputs
        .iter()
        .rev()
        .enumerate()
        .map(|(r, p)| {
            // dbg!(r + 1, p, p.r#type());
            (r + 1) as i32 * p.bid
        })
        .sum();
    // dbg!(res);

    Some(res.to_string())
}

pub fn two() -> Option<String> {
    None
}
