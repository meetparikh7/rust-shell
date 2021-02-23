# Extras

## `cd`

Our shell lacks a basic change directory command right now, lets add that:

Add to the TaskManager struct `cur_dir: PathBuf`

Add a `cd` method and use it in `execute`

```rust,no_run,noplayground
{{#include ../src/executor.rs:internal-cd}}
```

Then use `.current_dir(self.cur_dir.as_path())` in the command builder to launch the new command in this path.

## Future

This isnt complete and a lot more things can be done for learning.

Other features that can be added are:
- I/O redirection and pipes
- Variable expansion and environment variables
- Minor tweaks like `~` in paths
- More signal handling for a friendlier shell
- More shell built-ins

However I am satisfied with what I have learnt so far! I hope you learnt something new today too after reading this series!
