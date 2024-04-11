use std::{cmp::Ordering, io};

use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number!");

    let secret_number = generate_random_number();

    let guess = read_guess();

    println!("You guessed: {guess}");

    let result = match guess.cmp(&secret_number) {
        Ordering::Less => "Too small",
        Ordering::Greater => "Too big",
        Ordering::Equal => "You win",
    };

    println!("{result}!");
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
