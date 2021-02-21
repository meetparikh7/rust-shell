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
