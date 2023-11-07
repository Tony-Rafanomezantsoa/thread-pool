use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ThreadPoolCreationError;

impl Display for ThreadPoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThreadPool contains 0 thread")
    }
}

impl Error for ThreadPoolCreationError {}
