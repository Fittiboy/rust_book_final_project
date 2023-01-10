use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    sender: mpsc::Sender<Job>,
    _threads: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new `ThreadPool`
    ///
    /// The `size` parameter is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread count should be greater than zero!");

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32, Arc::clone(&receiver)));
        }

        ThreadPool {
            sender,
            _threads: threads,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    _id: u32,
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(_id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let _thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {_id} got a job; executing.");

            job();
        });
        Worker { _id, _thread }
    }
}
