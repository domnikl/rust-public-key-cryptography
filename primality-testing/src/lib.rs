pub mod prng;

use prng::Prng;

// Perform tests to see if a number is (probably) prime.
fn is_probably_prime(prng: &mut Prng, p: i64, num_tests: i64) -> bool {
    // Perform the tests.
    for _ in 0..num_tests {
        // Pick a number n in the range [1, p).
        let n = prng.next_i64(1, p);

        // Calculate n ^ (p - 1) mod p.
        let result = fast_exp_mod(n, p - 1, p);

        // If the final result is not 1, p is not prime.
        if result != 1 {
            return false;
        }
    }

    // If we survived all the tests, p is probably prime.
    return true;
}

// Probabilistically find a prime number within the range [min, max).
pub fn find_prime(prng: &mut Prng, min: i64, max: i64, num_tests: i64) -> i64 {
    // Try random numbers until we find a prime.
    loop {
        // Pick a random odd p.
        let p = prng.next_i64(min, max);
        if p % 2 == 0 {
            continue;
        }

        // See if it's prime.
        if is_probably_prime(prng, p, num_tests) {
            return p;
        }
    }
}

// Perform fast exponentiation in a modulus.
fn fast_exp_mod(mut num: i64, mut pow: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    while pow > 0 {
        if pow % 2 == 1 {
            result = (result * num) % modulus;
        }
        pow /= 2;
        num = (num * num) % modulus;
    }
    return result;
}

#[test]
fn is_prime() {
    let mut random = Prng::new();
    let num_tests = 20;
    let p = 31;
    assert!(is_probably_prime(&mut random, 31, num_tests))
}
