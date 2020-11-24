// Import read_line and flush
use std::io::{self, Write};

mod parser;

fn main() {
    // loop defines an infinite loop a.k.a. while (true)
    loop {
        // Create a mutable string
        let mut command = String::new();

        // print and flush the output for the prompt, ignore the `expect` method for now
        print!("$ ");
        io::stdout().flush().expect("Failed to write prompt");

        // Read the input, ignore the `expect` method for now
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        // If command is exit break from the loop
        if command.trim() == "exit" {
            break;
        };

        // Print the parsed command
        println!("You entered: {:?}", parser::parse(command.trim()));
    }
}
