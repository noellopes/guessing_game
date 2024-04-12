use std::cmp::Ordering;

use rand::{thread_rng, Rng};

mod utils;
use utils::read_u32;

fn main() {
    println!("Guess the number!");

    guess_number(generate_random_number());
}

fn guess_number(secret_number: u32) {
    let mut attempts: u32 = 0;
    loop {
        attempts += 1;
        println!("Attempt {attempts}");

        let guess = read_u32("Please input your guess:");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number {guess} is too small!"),
            Ordering::Greater => println!("The number {guess} is too big!"),
            Ordering::Equal => {
                println!("You win, after {attempts} attempts!");
                break;
            },
        };
    }
}

fn generate_random_number() -> u32 {
    let secret_number = thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    secret_number
}