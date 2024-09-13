// Import the `io` module from the standard library to handle input and output operations.
use std::io;

fn main() {
    // Print a message to the console indicating the start of the CLI calculator.
    println!("Simple CLI Calculator");
    // Prompt the user to enter an expression in the format "number operator number".
    println!("Enter an expression (e.g., 3 + 4):");

    // Enter an infinite loop to repeatedly read and process user input.
    loop {
        // Create a mutable `String` to store the user's input.
        let mut input = String::new();

        // Read a line of input from the standard input and store it in `input`.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim any leading or trailing whitespace from the input string.
        let trimmed_input = input.trim();

        // Check if the trimmed input is empty. If it is, print an error message and continue to the next iteration.
        if trimmed_input.is_empty() {
            println!("Invalid expression.");
            continue;
        }

        // Call the `evaluate_expression` function with the trimmed input and match on its result.
        match evaluate_expression(trimmed_input) {
            // If the result is `Ok`, print the result.
            Ok(result) => println!("Result: {}", result),
            // If the result is `Err`, print the error message.
            Err(err) => println!("{}", err),
        }
    }
}

// Define the `evaluate_expression` function to process and evaluate the mathematical expression.
fn evaluate_expression(expression: &str) -> Result<f64, &'static str> {
    // Split the expression into parts based on whitespace and collect them into a `Vec<&str>`.
    let parts: Vec<&str> = expression.split_whitespace().collect();

    // Check if the number of parts is exactly 3 (number operator number). If not, return an error.
    if parts.len() != 3 {
        return Err("Invalid input. Use the format 'number operator number'.");
    }

    // Parse the first part as an `f64` (floating-point number) and handle potential parsing errors.
    let num1: f64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return Err("Invalid input. Please enter a valid number for the first operand."),
    };

    // Store the operator (e.g., "+", "-", "*", "/") from the second part of the expression.
    let operator = parts[1];

    // Parse the second part as an `f64` (floating-point number) and handle potential parsing errors.
    let num2: f64 = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => return Err("Invalid input. Please enter a valid number for the second operand."),
    };

    // Match on the operator to determine the operation to perform, and return the result or an error.
    let result = match operator {
        "+" => Ok(num1 + num2), // Addition
        "-" => Ok(num1 - num2), // Subtraction
        "*" => Ok(num1 * num2), // Multiplication
        "/" => {
            // Check for division by zero and handle it with an error.
            if num2 == 0.0 {
                Err("Division by zero is not allowed.")
            } else {
                Ok(num1 / num2) // Division
            }
        }
        _ => Err("Invalid operator. Supported operators are +, -, *, /."), // Handle unsupported operators
    };

    // Return the result (if successful) or the error (if there was a problem).
    result
}
