# Signal handling

We want to handle simple UNIX signals. For learning we will handle `SIGTSTP` a.k.a. `Ctrl+Z` should send current process to background

Enter the [`signal-hook`](https://docs.rs/signal-hook/0.3.6/signal_hook/) crate, which handles UNIX signals in a Rust-friendly manner.

If you want to know more about the library, you can read the author's [blog post](https://vorner.github.io/2018/06/28/signal-hook.html). You can learn more about why UNIX signals isn't a great API [here](https://ldpreload.com/blog/signalfd-is-useless).

Add `signal-hook = "0.3.6"` as a dependency in `Cargo.toml`

## Registering signal handlers

The following lines allow us to register a `SIGTSTP` signal handler which will set the "flag variable" `shall_stop_process`

```rust,no_run,noplayground
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

let shall_stop_process = Arc::new(AtomicBool::new(false));
signal_hook::flag::register(signal_hook::consts::SIGTSTP, Arc::clone(&shall_stop_process)).unwrap();
```

This uses an [`AtomicBool`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html) as the signal handling occurs in another thread.

An `Arc` container is used by the `signal-hook` API, which is a thread-safe reference counted smart pointer - Read [Ch 15.4 Rc](https://doc.rust-lang.org/book/ch15-04-rc.html) and [Arc section in Ch 16.3](https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct) in the Rust Book for more information.

## Using this in the code

Lets look at this new method `run_process_in_fg` which waits for a process registering `SIGTSTP` handler

```rust,no_run,noplayground
{{#include ../src/executor.rs:run_process_in_fg}}
```

We then use this helper method in both `execute` and `fg`
