use std::cmp::Ordering;

use rand::{thread_rng, Rng};

mod utils;
use utils::read_u32;

const MIN: u32 = 1;
const MAX: u32 = 100;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(MIN..=MAX);

    guess_number(secret_number);
}

fn guess_number(secret_number: u32) {
    let mut attempts: u32 = 0;
    loop {
        attempts += 1;
        println!("Attempt {attempts}");

        let guess = read_u32("Please input your guess:", MIN, MAX);

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