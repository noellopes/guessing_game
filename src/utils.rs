use std::io;

pub fn read_u32(prompt: &str, min: u32, max: u32) -> u32 {
    loop {
        println!("{prompt}");
        let number = read_line();
        let number = number.trim();

        if let Ok(num) = number.parse() {
            if num >= min && num <= max {
                return num;
            } else {
                println!("Invalid number. Please input a number between {min} and {max}!");
            }
        } else {
            println!("The value '{number}' is not a number. Please input a number!");
        }
    }   
}

fn read_line() -> String {
    let mut guess = String::new();
    
    io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");

    guess
}