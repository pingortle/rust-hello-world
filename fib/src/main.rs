use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;
use std::vec;

fn main() {
    let mut memorized: Vec<BigUint> = vec![];

    loop {
        println!("Which Fibonacci number do you want?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("💀");

        let at: u32 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => 0,
        };

        if memorized.len() < at as usize {
            memorized.resize(at as usize, Zero::zero());
        }

        println!("The answer is {}", fib_memo(at, &mut memorized));
    }
}

fn fib(at: u32) -> BigUint {
    let mut memorized: Vec<BigUint> = vec![Zero::zero(); at as usize];

    if at <= 2 {
        One::one()
    } else {
        fib_memo(at - 1, &mut memorized) + fib_memo(at - 2, &mut memorized)
    }
}

fn fib_memo(at: u32, memorized: &mut Vec<BigUint>) -> BigUint {
    let index = at as usize - 1;

    if at <= 2 {
        One::one()
    } else if memorized[index] > Zero::zero() {
        memorized[index].clone()
    } else {
        let answer = fib_memo(at - 1, memorized) + fib_memo(at - 2, memorized);
        memorized[index] = answer.clone();
        answer
    }
}
