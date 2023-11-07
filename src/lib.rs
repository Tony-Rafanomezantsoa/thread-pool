mod error;

use std::{
    sync::{
        mpsc::{self, Sender, Receiver},
        Arc, Mutex,
    },
    thread::{JoinHandle, self},
};

use error::ThreadPoolCreationError;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job();
            }
        });

        Self { thread }
    }
}

/// `ThreadPool` is a type for executing tasks
/// concurrently.
pub struct ThreadPool {
    sender: Sender<Job>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new `ThreadPool` instance with `n` threads.
    ///
    /// # Error
    ///
    /// Returns [`Err`] if `n` is zero.
    pub fn create(n: usize) -> Result<Self, ThreadPoolCreationError> {
        if n == 0 {
            return Err(ThreadPoolCreationError);
        }

        let mut workers = Vec::with_capacity(n);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..n {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        Ok(Self { sender, workers })
    }
}
