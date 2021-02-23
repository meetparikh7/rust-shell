# Background jobs

This is a slightly meaty chapter

## TaskManager's state

First lets give our task manager the abilitiy to track background jobs. We will use a `HashMap` for this.

The `bg_jobs` HashMap tracks each child process and `cur_job` is used as an incrementing id

```rust
use std::collections::HashMap;

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
}
```

## Detecting background jobs

We say a job is to be run in background if it ends with an ampersand, e.g. `sleep 10 &`

<details>
<summary>Click here for code</summary>
So, we add in the execute block

```rust,no_run,noplayground
{{#include ../src/executor.rs:detect-bg-jobs}}
```
</details>

## Running a process in background

After spawning the process, if it is a background process, we simply add it to the `bg_jobs` HashMap with `cur_job` as the key and increment the the `cur_job`:

```rust,no_run,noplayground
if is_bg_job {
    self.cur_job += 1;
    self.bg_jobs.insert(self.cur_job, invoked_cmd);
    println!("Started bg job {}", self.cur_job);
}
```

## Bringing a background process back to the front using `fg`

We want to bring a process back to the front on running `fg <job_id>`. We move the child process out of the hashmap using the [`HashMap::remove`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.remove) method.

```rust,no_run,noplayground
{{#include ../src/executor.rs:internal-fg}}
```

We make sure that this method executes by putting at the top of teh execute method

```rust,no_run,noplayground
if base_cmd == "fg" {
    return self.fg(args);
}
```

## Checking on background jobs

We want a `jobs` command to print the list of the currently running jobs

```rust,no_run,noplayground
{{#include ../src/executor.rs:internal-jobs}}
```

Here, we first get mutable eferences to the child processes and call `ChildProcess::try_wait` on these. These allows us to know if the process has terminated or not. We also remove all terminated processes from the hashmap of jobs
