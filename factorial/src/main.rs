extern crate num;

use num::bigint::BigUint;
use num::traits::One;

enum Factorial {
    Result(String),
    InvalidNumber,
}

struct Computation<'a> {
    input: &'a String,
    output: Factorial,
}

fn big_factorial(n: u64) -> String {
    let mut r: BigUint = One::one();
    for i in 2..=n {
        r *= i;
    }
    r.to_string()
}

fn factorial(n: u64) -> String {
    if n < 17 {
        let mut r: u64 = 1;
        for i in 2..=n {
            if let Some(i) = r.checked_mul(i) {
                r = i;
            } else {
                // We overflowed, let's try with biguints instead
                return big_factorial(n);
            }
        }
        r.to_string()
    } else {
        big_factorial(n)
    }
}

fn compute(input: &String) -> Computation {
    let potential_number = input.parse();

    let output = if let Ok(number) = potential_number {
        Factorial::Result(factorial(number))
    } else {
        Factorial::InvalidNumber
    };

    Computation { input, output }
}

fn run_app() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        Err(())
    } else {
        args.iter().skip(1).map(compute).for_each(|x| {
            let result = match x.output {
                Factorial::Result(number) => number,
                Factorial::InvalidNumber => {
                    "Invalid number, expected an integer n where n >= 0 and n < 2^64.".into()
                }
            };
            println!("{}! = {}", x.input, result);
        });
        Ok(())
    }
}

fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(_err) => {
            eprintln!("Usage: factorial n1 [n2, ... nn]\nWhere n >= 0 and n < 2^64.\n\nDetails:");
            1
        }
    });
}
