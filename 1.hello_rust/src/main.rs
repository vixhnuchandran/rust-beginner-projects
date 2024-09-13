use std::io; // Importing the `io` module from the standard library. This module provides functionality to handle input/output operations, such as reading from the console.

fn main() {
    // The main function is the entry point of the program.

    let mut name = String::new(); // Create a mutable variable `name` and initialize it as an empty `String`.
                                  // This will hold the user's input (the name).

    println!("Please enter your name: "); // Print a message to the console prompting the user to enter their name.

    io::stdin() // Access the standard input (console input).
        .read_line(&mut name) // Read a line of input from the user and store it in the mutable `name` variable.
        .expect("Failed to read input"); // Handle any potential error during input. If an error occurs, this will print "Failed to read input" and stop the program.

    let name = name.trim(); // `trim()` removes any leading and trailing whitespace from the user's input (like the newline character).

    println!("Hello, {}!", name); // Use the `println!` macro to greet the user with their name. The `{}` is a placeholder for the value of `name`.
}
