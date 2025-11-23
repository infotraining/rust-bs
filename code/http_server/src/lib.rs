use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let message = receiver
                .lock()
                .unwrap()
                .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.", id = id);
                    job();
                },
                Err(_) => {
                    println!("Worker {id} shutting down.", id = id);
                    break;
                }
            }
        });


        Worker {id, thread: Some(thread)}
    }
}

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Option<std::sync::mpsc::Sender<Job>>,
}

#[derive(Debug)]
pub struct PoolCreationError(String);

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        assert!(size > 0);

        let (sender, receiver) = std::sync::mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {threads, sender: Some(sender)})
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().expect("Sender ERROR").send(job).unwrap();
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.threads {
            println!("Shutting down worker {id}", id = worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}