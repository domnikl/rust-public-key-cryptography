use primality_testing::find_prime;
use std::io::{self, Write};

use primality_testing::prng::Prng;

const NUM_TESTS: i64 = 20;

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    return str_value
        .trim()
        .parse::<i64>()
        .expect("Error parsing integer");
}

fn main() {
    // Prepare a Prng.
    let mut prng = Prng::new();

    // Display the probability that a number is prime
    // if it passes all NUM_TESTS tests.
    let probability = (1.0 - 1.0 / (2i64.pow(NUM_TESTS as u32)) as f64) * 100.0;
    println!("Probability: {}%\n", probability);

    // Generate random primes.
    loop {
        // Get the number of digits.
        let num_digits = get_i64("# Digits (max 9): ");
        if num_digits < 1 {
            break;
        }

        // Calculate minimum and maximum values.
        let mut min = 10i64.pow((num_digits - 1) as u32);
        let max = 10 * min;
        if min == 1 {
            min = 2;
        } // 1 is not prime.

        println!("Prime: {}", find_prime(&mut prng, min, max, NUM_TESTS));
    }
}
