use std::io;

fn main() {
    println!("Guess the number!");

    let guess = read_guess();

    println!("You guessed: {guess}");
}

fn read_guess() -> String {
    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}
