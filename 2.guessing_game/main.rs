use rand::Rng; // Importing the Rng trait from the rand crate, which allows us to use the random number generator.
use std::io; // Importing the standard input/output library to handle user input.

fn main() {
    // Creating a mutable random number generator that is local to the current thread.
    let mut rng = rand::thread_rng();

    // Generating a random number between 1 and 100 (inclusive) using the random number generator.
    let number = rng.gen_range(1..=100);

    // Printing an initial message to prompt the user to guess the number.
    println!("Guess the number!");

    // Creating a mutable string to store the user's input.
    let mut guess = String::new();

    // Starting a loop that will repeatedly ask for user input until the correct guess is made.
    loop {
        // Prompting the user to input their guess.
        println!("Please input your guess.");

        // Reading the user input from the standard input (stdin) and storing it in the 'guess' string.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input"); // Handling any errors that might occur when reading the input.

        // Trimming the string (to remove whitespace) and attempting to convert it to an unsigned integer (u32).
        // If successful, store the number in 'guess_num'; if unsuccessful, prompt the user to enter a valid number.
        let guess_num: u32 = match guess.trim().parse() {
            Ok(num) => num, // If the input is a valid number, store it in 'guess_num'.
            Err(_) => {
                // If parsing fails, handle the error:
                println!("Please enter a valid number!"); // Inform the user of invalid input.
                guess.clear(); // Clear the 'guess' string to prepare for the next input.
                continue; // Continue to the next iteration of the loop.
            }
        };

        // Comparing the user's guess to the randomly generated number:
        if guess_num < number {
            println!("Too small!"); // If the guess is too low, inform the user.
        } else if guess_num > number {
            println!("Too big!"); // If the guess is too high, inform the user.
        } else {
            println!("You guessed it!"); // If the guess is correct, congratulate the user.
            break; // Exit the loop since the game is over.
        }

        // Clear the 'guess' string so it can be reused for the next input.
        guess.clear();
    }
}
