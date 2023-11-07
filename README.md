# Thread Pool

A simple thread pool implementation for parallel task execution.

**NOTE:** This implementation is more for parallel rather than concurrent task.

# Usage

```rust
use std::{thread, time::Duration};

use threadpool::ThreadPool;

fn main() {
    // Create a thread pool with 4 threads.
    let mut pool = ThreadPool::create(4).unwrap();

    // Tasks / Subtasks to run in parallel.
    pool.new_task(|| println!("Action 1"));
    pool.new_task(|| {
        thread::sleep(Duration::from_secs(3));
        println!("Action 2");
    });
    pool.new_task(|| println!("Action 3"));

    // Execute all tasks in parallel and wait for completion.
    pool.execute();
}
```