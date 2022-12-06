static INPUT: &'static str = include_str!("./input");

pub fn one() -> Option<String> {
    INPUT
        .trim()
        .as_bytes()
        .windows(4)
        .enumerate()
        .filter(|w| {
            let mut s = std::str::from_utf8(w.1).unwrap().to_string();
            let mut seen = Vec::new();

            s.retain(|c| {
                let res = seen.contains(&c);
                seen.push(c);
                res
            });

            s.is_empty()
        })
        .map(|s| (s.0 + 4).to_string())
        .next()
}

pub fn two() -> Option<String> {
    INPUT
        .trim()
        .as_bytes()
        .windows(14)
        .enumerate()
        .filter(|w| {
            let mut s = std::str::from_utf8(w.1).unwrap().to_string();
            let mut seen = Vec::new();

            s.retain(|c| {
                let res = seen.contains(&c);
                seen.push(c);
                res
            });

            s.is_empty()
        })
        .map(|s| (s.0 + 14).to_string())
        .next()
}
