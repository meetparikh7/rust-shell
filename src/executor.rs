use std::collections::HashMap;
use std::process::{Child, Command, Stdio};

pub struct TaskManager {
    bg_jobs: HashMap<i64, Child>,
    cur_job: i64,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        // pass for now
        TaskManager {
            bg_jobs: HashMap::new(),
            cur_job: 0,
        }
    }

    pub fn execute(&mut self, command: Vec<String>) {
        if let Some((base_cmd, args)) = command.split_first() {
            let mut args = args; // Make args slice mutable

            // Check for internal commands
            if base_cmd == "fg" {
                return self.fg(args);
            } else if base_cmd == "jobs" {
                return self.jobs(args);
            }

            // Create a new command
            let mut cmd = Command::new(base_cmd);

            // ANCHOR: detect-bg-jobs
            // Detect if command ends with ampersand
            let is_bg_job = args.len() > 0
                && match args.last().unwrap().as_str() {
                    "&" => true,
                    _ => false,
                };
            if is_bg_job {
                // Remove the ampersand from the args
                args = &args[0..args.len() - 1]
            }
            // ANCHOR_END: detect-bg-jobs

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

            if is_bg_job {
                self.cur_job += 1;
                self.bg_jobs.insert(self.cur_job, invoked_cmd);
                println!("Started bg job {}", self.cur_job);
            } else {
                // Wait for command to finish
                invoked_cmd
                    .wait_with_output()
                    .expect(&format!("Failed to wait for {}", base_cmd));
            }
        } else {
            // Pass as empty command
        }
    }

    // ANCHOR: internal-fg
    fn fg(&mut self, args: &[String]) {
        let job = args[0].parse::<i64>().unwrap();
        // Move the child_process out of the hashmap
        let child_process = self.bg_jobs.remove(&job).unwrap();
        // Wait for command to finish
        child_process
            .wait_with_output()
            .expect(&format!("Failed to wait for background job {}", job));
    }
    // ANCHOR_END: internal-fg

    // ANCHOR: internal-jobs
    fn jobs(&mut self, _args: &[String]) {
        let mut toremove: Vec<i64> = vec![];
        for (job, child_process) in self.bg_jobs.iter_mut() {
            match child_process
                .try_wait()
                .expect(&format!("Was unable to check status of bg job {}", job))
            {
                Some(status) => {
                    println!("Job {} exited with status {}", job, status);
                    toremove.push(*job);
                }
                None => println!("Job {} still running", job),
            }
        }
        for job in toremove {
            self.bg_jobs.remove(&job);
        }
    }
    // ANCHOR_END: internal-jobs
}
