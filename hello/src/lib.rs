use std::{
    error,
    fmt::Display,
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread::{self},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for index in 0..size {
            workers.push(Worker::new(index, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    //pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //    if size < 1 {
    //        return Err(PoolCreationError);
    //    }
    //
    //      let mut workers = Vec::with_capacity(size);
    //
    //      for index in 0..size {
    //        workers.push(Worker::new(index));
    //      }
    //      Ok(ThreadPool { workers })
    // }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker [{id}]: Got a Job to execute.");
                    job();
                }
                Err(_) => {
                    println!("Worker [{id}] disconnected; shutting down");
                    break;
                }
            };
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct PoolCreationError;

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error has ocurred while the Thread Poll has been created"
        )
    }
}

impl error::Error for PoolCreationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}
