use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    sender: Sender<Job>,
}

impl Worker {
    fn new(id: usize) -> Self {
        let (sender, receiver): (mpsc::Sender<Job>, mpsc::Receiver<Job>) = mpsc::channel();
        thread::spawn(move || {
            for job in receiver {
                println!("Worker {id} got a job. Executing it.");
                job();
            }
        });

        Self { id, sender }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    next_idx: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size == 8); // =) fixed for now

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }

        Self {
            workers,
            next_idx: 0,
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        const LAST_ENTRY: usize = 0b111;
        let job = Box::new(f);
        self.workers[self.next_idx].sender.send(job).unwrap();
        self.next_idx = (self.next_idx + 1) & LAST_ENTRY;
    }
}
