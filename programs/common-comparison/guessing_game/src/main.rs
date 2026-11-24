use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // :: syntax in the ::new line indicates that new is an associated function of the String type.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing the previous value of guess with a new one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater=> println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }   
}
