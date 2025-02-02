use signal_hook;
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::env;
use std::path::PathBuf;

pub struct TaskManager {
    bg_jobs: HashMap<i64, Child>,
    cur_job: i64,
    cur_dir: PathBuf
}

impl TaskManager {
    pub fn new() -> TaskManager {
        // pass for now
        TaskManager {
            bg_jobs: HashMap::new(),
            cur_job: 0,
            cur_dir: env::current_dir().unwrap()
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
            } else if base_cmd == "cd" {
                return self.cd(args);
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
                .current_dir(self.cur_dir.as_path())
                .spawn()
                .expect(&format!("Failed to start {}", base_cmd));

            if is_bg_job {
                self.cur_job += 1;
                self.bg_jobs.insert(self.cur_job, invoked_cmd);
                println!("Started bg job {}", self.cur_job);
            } else {
                self.run_process_in_fg(invoked_cmd);
            }
        } else {
            // Pass as empty command
        }
    }

    // ANCHOR: run_process_in_fg
    fn run_process_in_fg(&mut self, mut child_process: Child) {
        let shall_stop_process = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(
            signal_hook::consts::SIGTSTP,
            Arc::clone(&shall_stop_process),
        )
        .unwrap();

        loop {
            if shall_stop_process.load(Ordering::Relaxed) {
                self.cur_job += 1;
                self.bg_jobs.insert(self.cur_job, child_process);
                println!("Now running as background job {}", self.cur_job);
                break;
            }
            // Wait for command to finish
            match child_process
                .try_wait()
                .expect(&format!("Failed to wait for current job"))
            {
                Some(_status) => break,
                _ => (),
            }
        }
    }
    // ANCHOR_END: run_process_in_fg

    // ANCHOR: internal-fg
    fn fg(&mut self, args: &[String]) {
        let job = args[0].parse::<i64>().unwrap();
        // Move the child_process out of the hashmap
        let child_process = self.bg_jobs.remove(&job).unwrap();
        // Wait for command to finish
        self.run_process_in_fg(child_process);
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

    // ANCHOR: internal-cd
    fn cd(&mut self, args: &[String]) {
        let new_dir = std::path::Path::new(&args[0]);
        let new_dir_pathbuf;
        if new_dir.is_relative() {
            new_dir_pathbuf = self.cur_dir.join(new_dir);
        } else {
            new_dir_pathbuf = new_dir.to_path_buf();
        }
        self.cur_dir = new_dir_pathbuf;
    }
    // ANCHOR_END: internal-cd
}
