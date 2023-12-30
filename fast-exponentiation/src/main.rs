use std::io::{self, Write};

use fast_exponentiation::{fast_exp, fast_exp_mod};

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
    let mut num: i64;
    let mut pow: i64;
    let mut modulus: i64;

    loop {
        num = get_i64("num: ");
        pow = get_i64("pow: ");
        modulus = get_i64("modulus: ");

        let exp = fast_exp(num, pow);
        let exp_mod = fast_exp_mod(num, pow, modulus);

        assert!(exp == num.pow(pow as u32));
        assert!(exp_mod == (num.pow(pow as u32) % modulus));

        println!("{}^{} = {}", num, pow, exp);
        println!("{}^{} mod {} = {}", num, pow, modulus, exp_mod);
    }
}
