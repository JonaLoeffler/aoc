use std::env;
use std::fs;

fn main() {
    // Read the first command line argument
    let part = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: missing part argument");
        std::process::exit(1);
    });

    // Parse the part argument
    let part = part.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Error: invalid part argument");
        std::process::exit(1);
    });

    // Read the input string
    let input = fs::read_to_string("input.txt").unwrap();
    let s = input.trim();

    // Keep track of the last 4 or 14 characters
    let mut last_four_or_fourteen = Vec::new();

    // Iterate over the characters in the string
    for (i, c) in s.chars().enumerate() {
        // Append the current character to the last_four_or_fourteen vector
        last_four_or_fourteen.push(c);

        // If the length of the set of the last 4 or 14 characters is 4 or 14,
        // then we have found the first start-of-packet or start-of-message marker
        if last_four_or_fourteen.iter().collect::<std::collections::HashSet<_>>().len() == part {
            // Print the result
            println!("{}", i + 1);
            break;
        }

        // If the length of the last_four_or_fourteen vector is more than 4 or 14, remove
        // the first element from the vector
        if last_four_or_fourteen.len() >= part {
            last_four_or_fourteen.remove(0);
        }
    }
}
