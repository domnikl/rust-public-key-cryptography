use factoring_numbers::{find_factors, find_factors_sieve, sieve_of_eratosthenes, sieve_to_primes};
use std::io::{self, Write};

const MAX_PRIME: usize = 100_000_000;

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
    let mut primes = sieve_to_primes(sieve_of_eratosthenes(MAX_PRIME));
    let mut num: i64;

    loop {
        num = get_i64("num: ");

        let factors = find_factors(num);
        let factors2 = find_factors_sieve(&mut primes, num);

        println!(
            "factors of {}: {}",
            num,
            factors
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        println!(
            "factors of {}: {} (sieve)",
            num,
            factors2
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
