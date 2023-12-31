use std::io::{self, Write};

fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut sieve = vec![true; max];

    // Set 0 and 1 to false, they are not primes.
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..max {
        if sieve[i] {
            for j in (i * i..max).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    sieve
}

fn print_sieve(sieve: &mut Vec<bool>) {
    print!("2 ");

    // save time by only iterating over odd-numbered items
    for (i, is_prime) in sieve.iter_mut().enumerate().skip(3).step_by(2) {
        if *is_prime {
            print!("{} ", i);
        }
    }

    println!();
}

// Convert the sieve into a vector holding prime numbers.
fn sieve_to_primes(sieve: Vec<bool>) -> Vec<i64> {
    let mut primes = Vec::new();

    for (i, is_prime) in sieve.iter().enumerate() {
        if *is_prime {
            primes.push(i as i64);
        }
    }

    primes
}

// Print the vector of numbers.
fn print_numbers(primes: &mut Vec<i64>) {
    for prime in primes {
        print!("{prime} ");
    }
    println!();
}

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
    let max = get_i64("Max: ");
    let mut sieve = sieve_of_eratosthenes(max as usize);
    if max < 1000 {
        print_sieve(&mut sieve);
    }

    let mut primes = sieve_to_primes(sieve);
    if max < 1000 {
        print_numbers(&mut primes);
    }
}

#[test]
fn test_sieve_of_eratosthenes() {
    let sieve = sieve_of_eratosthenes(100);
    let primes = sieve_to_primes(sieve);
    assert_eq!(
        primes,
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    );
}
