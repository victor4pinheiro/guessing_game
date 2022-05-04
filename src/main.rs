use std::io;

pub fn main() {
    println!("Guess the number!");
    println!("Please input the number:");

    // create a mutable variable of type string and set :: (associated function that's implemented on a type)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
