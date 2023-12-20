use std::collections::HashMap;

static INPUT: &'static str = include_str!("./input");

fn parse() -> HashMap<String, Module> {
    let mut res: HashMap<String, Module> = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(" -> ");

            let start = split.next().unwrap();
            let kind = match start {
                "broadcaster" => Kind::Broadcaster,
                s if s.starts_with("&") => Kind::Conjunction(HashMap::new()),
                s if s.starts_with("%") => Kind::FlipFlop(State::Off),
                s => panic!("{s}"),
            };

            let start = start.replace("&", "").replace("%", "");

            let next = split
                .next()
                .unwrap()
                .split(", ")
                .map(ToString::to_string)
                .collect();

            (start.to_string(), Module { kind, next })
        })
        .collect();

    for (name, module) in &res.clone() {
        // if next contains a conjunction module, at this module to the conjuction modules list
        // of previous
        for n in &module.next {
            if let Some(target) = res.get_mut(n) {
                if let Kind::Conjunction(ref mut next) = target.kind {
                    next.insert(name.to_string(), Pulse::Low);
                }
            }
        }
    }

    res
}

#[derive(Debug, Clone)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Module {
    kind: Kind,
    next: Vec<String>,
}

#[derive(Debug, Clone)]
enum Kind {
    Broadcaster,
    FlipFlop(State),
    Conjunction(HashMap<String, Pulse>),
}

#[derive(Debug, Clone)]
enum Pulse {
    High,
    Low,
}

fn process(
    from: &str,
    pulse: Pulse,
    to: &str,
    modules: &mut HashMap<String, Module>,
) -> Vec<(String, Pulse, String)> {
    // println!("{from} -{pulse:?}-> {to}");

    let mut send: Vec<(String, Pulse, String)> = Vec::new();

    if let Some(module) = modules.get_mut(to) {
        match module.kind {
            Kind::Broadcaster => {
                send = module
                    .next
                    .iter()
                    .map(|n| (to.to_owned(), pulse.to_owned(), n.to_owned()))
                    .collect()
            }
            Kind::FlipFlop(ref mut s) => {
                if let Pulse::Low = pulse {
                    *s = match s {
                        State::Off => State::On,
                        State::On => State::Off,
                    };

                    send = module
                        .next
                        .iter()
                        .map(|n| {
                            (
                                to.to_owned(),
                                match s {
                                    State::On => Pulse::High,
                                    State::Off => Pulse::Low,
                                },
                                n.to_owned(),
                            )
                        })
                        .collect()
                }
            }
            Kind::Conjunction(ref mut prev) => {
                prev.insert(from.to_string(), pulse);

                let res = if prev.values().all(|p| match p {
                    Pulse::High => true,
                    Pulse::Low => false,
                }) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                send = module
                    .next
                    .iter()
                    .map(|n| (to.to_owned(), res.to_owned(), n.to_owned()))
                    .collect();
            }
        }
    }

    send
}

pub fn one() -> Option<String> {
    let mut modules = parse();

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut queue = vec![("button".to_string(), Pulse::Low, "broadcaster".to_string())];

        while let Some((from, pulse, to)) = queue.pop() {
            match pulse {
                Pulse::High => high += 1,
                Pulse::Low => low += 1,
            }

            for item in process(&from, pulse, &to, &mut modules) {
                queue.insert(0, item);
            }
        }
    }

    Some((low * high).to_string())
}

pub fn two() -> Option<String> {
    let mut modules = parse();

    let mut dl: Vec<u64> = Vec::new();
    let mut pm: Vec<u64> = Vec::new();
    let mut vk: Vec<u64> = Vec::new();
    let mut ks: Vec<u64> = Vec::new();

    for i in 0.. {
        let mut queue = vec![("button".to_string(), Pulse::Low, "broadcaster".to_string())];

        while let Some((from, pulse, to)) = queue.pop() {
            match (pulse.clone(), from.as_str()) {
                (Pulse::High, "dl") => dl.push(i),
                (Pulse::High, "pm") => pm.push(i),
                (Pulse::High, "vk") => vk.push(i),
                (Pulse::High, "ks") => ks.push(i),
                _ => (),
            }

            let dl: Option<u64> = dl.windows(2).map(|w| w[1] - w[0]).next();
            let pm: Option<u64> = pm.windows(2).map(|w| w[1] - w[0]).next();
            let vk: Option<u64> = vk.windows(2).map(|w| w[1] - w[0]).next();
            let ks: Option<u64> = ks.windows(2).map(|w| w[1] - w[0]).next();

            let res = dl.unwrap_or_default()
                * pm.unwrap_or_default()
                * vk.unwrap_or_default()
                * ks.unwrap_or_default();
            if res > 0 {
                return Some(res.to_string());
            }

            if let (Pulse::Low, "rx") = (&pulse, to.as_str()) {
                return Some(i.to_string());
            }

            for item in process(&from, pulse, &to, &mut modules) {
                queue.insert(0, item);
            }
        }
    }

    None
}

mod tests {}
