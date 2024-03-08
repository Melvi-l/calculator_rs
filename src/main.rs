use std::io::{self, Write};

use calc_lib::calc;

pub fn main() {
    println!("Hello world");
    let mut input = String::new();

    loop {
        print!("calc >> ");
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let resultat = calc(input.trim());
        println!("Result: {}", resultat);

        input.clear();
    }
}


