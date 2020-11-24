# Introduction

I aim to make a small bash-like shell in Rust both as a learning exercise and for fun!

I am also documenting this as a tutorial for others to follow! The prerequisites for this tutorial would be a basic knowledge of some low-level programming language like C/C++ or Rust.

## Getting started

- Make the project directory - `mkdir rust-shell && cd rust-shell`
- Init the project - `cargo init` - `cargo` is Rust's inbuilt package manager (like `npm`)
- Run the project - `cargo run`. This should print "Hello, world!"
- (I followed an additional step after this to setup this tutorial)

## Accepting input

Now lets try to accept a bit of input and echo it back

Modify `src/main.rs` to:

```rust
// Import read_line and flush
use std::io::{self, Write};

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

        // Print the entered command
        println!("You entered: {}", command);
    }
}
```

## Note about `Result` and `expect`

This is Rust's way of exception handling.

**TLDR** we want to `panic!` and quit the program if the methods return an `Err`, else continue

Rust's exception system might be slightly new to C++ users, but I will try to summarize it here.

- If you think your method can throw an error, you must return a `Result<T, E>` instead of `T` where `T` is the type you intended to return initially and `E` is the error type (say `std::invalid_argument` from C++)
- A `Result<T, E>` is either a `Ok(T)` or an `Err(E)`. `T` and `E` can be any types.
- So the return type for `flush` is `Result<()>`. (`()` is the `void` type.)
- i.e. it either returns `Ok` or `Err`
- We want to cause a unrecoverable runtime error when we cant flust the stdout due to some system reason
- You may chose to `panic!` for unrecoverable runtime errors, which quits the program instantly
- `expect(str)` is a helper method in `Result`, which calls `panic!(str)` if an `Err` was returned from the method or continues otherwise

You can read more about this from the [Rust Book](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
