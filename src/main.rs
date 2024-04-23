use std::time::{SystemTime, UNIX_EPOCH};

const ALLOWED_CHARS: &str = "hBsjPJa5Llm";

fn main() {
    // Get the hour in ms
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seed = since_the_epoch.as_secs();

    // Generate random big number
    let mut random_number = String::new();

    for _ in 0..16 {
        let state = (seed >> 1) | ((seed & 1) ^ (seed << 1 & 1)) << 15; // Implementation of LSFR state

        // Add all the state, because if I add just the last, always returns 0
        random_number += (state*2).to_string().as_str();
    }

    let mut result = String::new();

    for char in random_number.chars() {
        let number = char.to_string().trim().parse::<usize>().unwrap();
        result += ALLOWED_CHARS.chars().nth(number).unwrap().to_string().as_str();
    }

    println!("{result}");
}
