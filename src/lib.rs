use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {})
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
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // with_capacity preallocate space in the vector
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    // The signature is inspired by the spawn signature
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
