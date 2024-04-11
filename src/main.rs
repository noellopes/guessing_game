use std::io;

use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number!");

    let _secret_number = generate_random_number();

    let guess = read_guess();

    println!("You guessed: {guess}");
}

fn generate_random_number() -> u32 {
    let secret_number = thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    secret_number
}

fn read_guess() -> String {
    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}
