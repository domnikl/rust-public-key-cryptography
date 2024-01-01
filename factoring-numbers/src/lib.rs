// Convert the sieve into a vector holding prime numbers.
pub fn sieve_to_primes(sieve: Vec<bool>) -> Vec<i64> {
    let mut primes = Vec::new();

    for (i, is_prime) in sieve.iter().enumerate() {
        if *is_prime {
            primes.push(i as i64);
        }
    }

    primes
}

pub fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
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

pub fn find_factors(mut num: i64) -> Vec<i64> {
    // find all factors of n
    let mut factors = Vec::new();

    while (num % 2) == 0 {
        factors.push(2);
        num /= 2;
    }

    // Take out other primes
    let mut factor = 3;
    while factor * factor <= num {
        if num % factor == 0 {
            factors.push(factor);
            num /= factor;
        } else {
            // go to the next odd number
            factor += 2;
        }
    }

    if num != 1 {
        factors.push(num);
    }

    factors
}

pub fn find_factors_sieve(primes: &mut Vec<i64>, mut num: i64) -> Vec<i64> {
    let mut factors = Vec::new();

    if num < 0 {
        factors.push(-1);
        num = -num;
    }

    for prime in primes {
        let factor = *prime;
        while num % factor == 0 {
            factors.push(factor);
            num /= factor;
            if num == 1 {
                break;
            }
        }
        if factor * factor > num {
            factors.push(num);
            break;
        }
    }

    return factors;
}

#[test]
fn find_factors_without_sieve() {
    assert_eq!(find_factors(312680865509917), vec![7791799, 40129483]);
    assert_eq!(find_factors(1819448968910731), vec![40129483, 45339457]);
    assert_eq!(find_factors(12345678901234), vec![2, 7, 73, 12079920647]);
    assert_eq!(find_factors(6795742697625173), vec![6880691, 987654103]);
    assert_eq!(find_factors(64374108854777), vec![64374108854777]);
}

#[test]
fn find_factors_with_sieve() {
    let mut primes = sieve_to_primes(sieve_of_eratosthenes(1000));

    assert_eq!(
        find_factors_sieve(&mut primes, 123123),
        vec![3, 7, 11, 13, 41]
    );
}
