// prelude
// If a type you want isn't in the prelude, you have to bring that type into scope explicitly with a statement.
// std::io provides a number of useful features, include the ability to accept user input.
use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    // Print prompts stating what the game is and requesting input from the user.
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Storing Values with Variables
        // let statement creates the variable.
        // In Rust, variables are immutable by default - Once we give a variable a value, that value won't change.
        // adding mut before a variable makes it mutable.
        let mut guess = String::new(); // :: syntax in the ::new line indicates that new is an associated function of the String type.

        // An associated funciton is a funciton that's implemented ona type, in this case: String.

        // std::io provied input/output functionality. The stdin function from the io module can be seen below to handle user input:
        // Like in C++, if we didn't use std::io above, we could still use the function calll as std::io::stdin
        // stdin returns an instance of std::io::Stdin(a type that represents a handle to the standard input of my terminal)
        io::stdin()
            // Full job of read_line() is to take whatever the user types into std in and append that into a string (w/o overwriting its contents)
            // The string argument needs to be mutable so that the method can change the string's content.
            // & indicates that this argument is a reference which allows away to let multiple parts of program access one piece of data w/o
            // needing to copy that data into memory multiple times. References are immutable by default.
            .read_line(&mut guess)
            // An instance of Result has an expect method that you can call.
            // If instance of Resultis Err value, expect will cause program to crash, and display the message that you passed
            // as an argument of expect.
            // If .read_line method returns Err, it is likely a result of the error coming form th eOS.
            // If this instance of Result is an ok value, expet will take on the return value that ok is holding and return it.
            // In this instance, it is the number of bytes in the user's input.
            .expect("Failed to read line");

        // We are shadowing the previous value of guess with a new one.
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
