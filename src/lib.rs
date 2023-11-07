use std::{
    error::Error,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Task = Box<dyn FnOnce() + Send + 'static>;

/// `ThreadPool` is a type for
/// running multiple task in parallel.
pub struct ThreadPool {
    threads: usize,
    tasks: Vec<Task>,
}

impl ThreadPool {
    /// Create a new `ThreadPool` instance with `n` thread.
    ///
    /// # Error
    ///
    /// Returns [Err] if `n` is zero.
    pub fn create(n: usize) -> Result<Self, Box<dyn Error>> {
        if n == 0 {
            return Err(From::from("Invalid number of thread"));
        }

        Ok(Self {
            threads: n,
            tasks: Vec::new(),
        })
    }

    /// Add a new task to execute.
    pub fn new_task<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tasks.push(Box::new(f));
    }

    /// Execute and wait for all tasks to finish, according to the
    /// number of thread in the `ThreadPool`.
    pub fn execute(self) {
        let (sender, receiver) = mpsc::channel();

        for task in self.tasks {
            sender.send(task).unwrap();
        }

        let mut workers = Vec::new();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..self.threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        //Ensure that all task is finished.
        for worker in workers {
            let _ = worker.handle.join();
        }
    }
}

struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Task>>>) -> Self {
        let thread = thread::Builder::new().name(format!("{}", id));
        let handle = thread
            .spawn(move || loop {
                let receiver = receiver.lock().unwrap();

                let task = match receiver.try_recv() {
                    Ok(t) => t,
                    _ => break,
                };

                drop(receiver);
                task();
            })
            .unwrap();

        Self { handle }
    }
}
