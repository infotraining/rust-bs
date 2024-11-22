use std::{sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread::JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Task>>,
}

type Task = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        let mut workers = Vec::with_capacity(size as usize);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{ workers, 
            sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let task = Box::new(f);
        self.sender.
            as_ref().unwrap().send(task).unwrap();
    }
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

struct Worker {
    id: u32,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<Receiver<Task>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let message = receiver
                .lock()
                .unwrap()
                .recv();

            match message {
                Ok(task) => {
                    println!("Worker {} got a task; executing.", id);
                    task();
                },
                Err(_) => {
                    println!("Worker {} is shutting down.", id);
                    break;
                }
            }            
        });

        Worker { 
            id, 
            thread: Some(thread) 
        }
    }
}