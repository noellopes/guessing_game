use std::io;

pub fn read_u32(prompt: &str) -> u32 {
    loop {
        println!("{prompt}");
        let number = read_line();
        let number = number.trim();

        match number.parse() {
            Ok(num) => return num,
            Err(_) => println!("The value '{number}' is not a number. Please input a number!"),
        };
    }   
}

fn read_line() -> String {
    let mut guess = String::new();
    
    io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");

    guess
}