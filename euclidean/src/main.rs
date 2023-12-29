use std::io::{self, Write};

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    return str_value.trim().parse::<i64>()
        .expect("Error parsing integer");
}

fn gcd(a: i64, b: i64) -> i64 {
    return if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        let x = if a < 0 { -a } else { a };
        let y = if b < 0 { -b } else { b };

        gcd(y, x % y)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn main() {
    println!("  A  B  GCD(A, B)  LCM(A, B)");
    let mut a: i64;
    let mut b: i64;

    loop {
        a = get_i64("A: ");
        b = get_i64("B: ");

        println!("{} {} {} {}", a, b, gcd(a, b), lcm(a, b));
    }
}

#[test]
fn gcd_valid() {
    assert_eq!(6, gcd(270, 192));
    assert_eq!(6, gcd(-270, 192));
    assert_eq!(77, gcd(7469, 2464));
    assert_eq!(970, gcd(55290, 115430));
}

#[test]
fn lcm_valid() {
    assert_eq!(8640, lcm(270, 192));
    assert_eq!(8640, lcm(-270, -192));
    assert_eq!(239008, lcm(7469, 2464));
    assert_eq!(6579510, lcm(55290, 115430));
}
