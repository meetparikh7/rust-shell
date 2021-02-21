# Simple command execution

Lets now try to execute the command

The docs for the [`std::process`](https://doc.rust-lang.org/std/process/index.html) module tell us exactly how to do this

We create a new module called `executor` and add the following code there:

```rust
use std::process::{Command, Stdio};

pub struct TaskManager {}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {}
    }

    pub fn execute(&mut self, command: Vec<String>) {
        if let Some((base_cmd, args)) = command.split_first() {
            // Create a new command
            let mut cmd = Command::new(base_cmd);
            // Add args to the command
            for arg in args {
                cmd.arg(arg);
            }

            // Invoke command with stdin and stdout of parent, i.e. the shell
            let invoked_cmd = cmd
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()
                .expect(&format!("Failed to start {}", base_cmd));

            // Wait for command to finish
            invoked_cmd
                .wait_with_output()
                .expect(&format!("Failed to wait for {}", base_cmd));
        } else {
            // Pass as empty command
        }
    }
}
```

Stepping over it line-by-line:
- We create an empty struct called `TaskManager` and have an `execute` method in it. This struct will come handy later.
- Here, we note we use `split_first` to get a tuple telling us the base command and the remaining args
- We first create a new mutable `std::process::Command`
- Then we add all the args one-by-one using `.arg` method. Doing this one-by-one will allow us to easily substitute variables in the future
- Finally, we invoke it saying inherit `stdin` and `stdout` from the parent. This means if the shell was run interactively, the stdin and stfout will be interactive but if file redirection was done while launching the shell itself, then it will use the redirected input/output.
- Finally, we wait for it to finish and then return.

We modify `main.rs` to run this `execute` method instead of printing the tokenized command:

```rust
mod executor;

// ...

let mut task_manager = executor::TaskManager::new();

// ...

match parser::parse(command.trim()) {
    Some(parsed_command) => task_manager.execute(parsed_command),
    None => println!("Invalid command")
}
```
