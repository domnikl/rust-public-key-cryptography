pub fn fast_exp(mut num: i64, mut pow: i64) -> i64 {
    let mut result = 1;

    while pow > 0 {
        if pow % 2 == 1 {
            result *= num;
        }

        pow /= 2;
        num *= num;
    }

    result
}

pub fn fast_exp_mod(mut num: i64, mut pow: i64, modulus: i64) -> i64 {
    let mut result = 1;

    while pow > 0 {
        if pow % 2 == 1 {
            result = (result * num) % modulus;
        }

        pow /= 2;
        num = (num * num) % modulus;
    }

    result
}

#[test]
fn test_fast_exp() {
    assert_eq!(fast_exp(2, 3), 8);
    assert_eq!(fast_exp(5, 0), 1);
    assert_eq!(fast_exp(10, 5), 100000);
    assert_eq!(fast_exp(3, 4), 81);
    assert_eq!(fast_exp(7, 2), 49);
}

#[test]
fn test_fast_exp_mod() {
    assert_eq!(fast_exp_mod(8, 5, 10), 8);
    assert_eq!(fast_exp_mod(8, 6, 10), 4);
}
