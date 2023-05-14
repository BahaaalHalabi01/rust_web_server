use std::{thread, sync::{mpsc::{self, Receiver}, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender:mpsc::Sender<Job>
}

impl ThreadPool {
    //Creates a new thread pool
    //size is the number of threads in the pool
    //
    //#Panics
    //if size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender,reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers= Vec::with_capacity(size);
        for id  in 0..size {
            workers.push(Worker::new(id,Arc::clone(&reciever)))
        }

        ThreadPool {workers,sender}
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
    id:usize,
    thread:thread::JoinHandle<()>
}

impl Worker {
    fn new(id:usize,reciever:Arc<Mutex<Receiver<Job>>>)-> Self {
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.",id);

            job();
        });
        Worker { id,thread}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

