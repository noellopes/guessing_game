use std::{cmp::Ordering, io};

use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number!");

    guess_number(generate_random_number());
}

fn guess_number(secret_number: u32) {
    let mut attempts: u32 = 0;
    loop {
        attempts += 1;
        println!("Attempt {attempts}");

        let guess = read_guess();

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

fn read_guess() -> u32 {
    println!("Please input your guess.");    
    let guess = read_line();
    
    guess
        .trim()
        .parse()
        .expect("Please type a number!")
}

fn read_line() -> String {
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}
