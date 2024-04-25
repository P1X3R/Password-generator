use std::time::{SystemTime, UNIX_EPOCH};

const ALLOWED_CHARS: &str = "jGcA8g2v=q/D";

fn main() {
    // Read the input
    let mut password_length = String::new();

    println!("Enter the password length: ");
    std::io::stdin().read_line(&mut password_length).unwrap();
    let password_length: usize = password_length.trim().parse().unwrap();

    // Get the hour in ms
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seed = since_the_epoch.as_secs();

    // Generate random big number
    let mut random_number = String::new();
    let mut operation: &str = "multiply";

    for _ in 0..password_length {
        let state = (seed >> 1) | ((seed & 1) ^ (seed << 1 & 1)) << 15; // Implementation of LSFR state

        // Add all the state, because if I add just the last, always returns 0
        if operation == "multiply" {
            random_number += (state*2).to_string().as_str();
            operation = "divition";
        }
        else if operation == "divition" {
            random_number += (state/2).to_string().as_str();
            operation = "power";
        }
        else if operation == "power" {
            random_number += (state*2*2*2).to_string().as_str();
            operation = "multiply";
        }
    }

    let mut result = String::new();

    for char in random_number.chars() {
        let number = char.to_string().trim().parse::<usize>().unwrap();
        result += ALLOWED_CHARS.chars().nth(number).unwrap().to_string().as_str();
    }

    let result = &(result.as_str())[..password_length];
    println!("Result: {result}");
}
