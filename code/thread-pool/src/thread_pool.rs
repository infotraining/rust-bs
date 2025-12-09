use std::fmt;
use std::sync::{Arc, Mutex, mpsc};

use tokio::sync::oneshot;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = std::thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(task) => {
                        task();
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Debug)]
pub struct ThreadPoolError {
    details: String,
}

impl fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for ThreadPoolError {}

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size == 0 {
            return Err(ThreadPoolError {
                details: String::from("Size must be greater than zero."),
            });
        }

        let (sender, receiver) = mpsc::channel();
        let shared_receiver = std::sync::Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            let receiver_cloned = shared_receiver.clone();
            threads.push(Worker::new(id, receiver_cloned));
        }

        Ok(ThreadPool {
            threads,
            sender: Some(sender),
        })
    }

    pub fn submit<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        let sender = self.sender.as_ref().ok_or(ThreadPoolError {
            details: String::from("ThreadPool has no sender."),
        })?;
        sender.send(job).map_err(|_| ThreadPoolError {
            details: String::from("Failed to send job to the thread pool."),
        })?;
        Ok(())
    }

    pub fn submit_tokio<F, T>(&self, f: F) -> Result<oneshot::Receiver<T>, ThreadPoolError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel::<T>();

        let job = Box::new(move || {
            let res = f();
            // ignore send error: receiver may have been dropped
            let _ = tx.send(res);
        }) as Job;

        let sender = self.sender.as_ref().ok_or(ThreadPoolError {
            details: "ThreadPool has no sender.".to_string(),
        })?;

        sender.send(job).map_err(|_| ThreadPoolError {
            details: "Failed to send job to the thread pool.".to_string(),
        })?;

        Ok(rx)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // closing the channel - no more messages can be sent - workers will break out of their loop

        for worker in &mut self.threads {
            if let Some(thread) = worker.thread.take() {
                if let Err(err) = thread.join() {
                    eprintln!("Worker {} panicked during shutdown: {:?}", worker.id, err);
                }
            }
        }
    }
}

#[cfg(test)]
mod thread_pool_tests {
    use std::os::windows::process;

    use super::*;

    #[test]
    fn thread_pool_with_size_zero_fails() {
        let result = ThreadPool::new(0);
        match result {
            Err(e) => assert_eq!(e.details, "Size must be greater than zero."),
            Ok(_) => panic!("ThreadPool creation with size 0 should have failed."),
        }
    }

    #[test]
    fn thread_pool_executes_tasks() {
        let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
        {
            let thread_pool = ThreadPool::new(4).expect("Failed to create thread pool");

            for _ in 0..100 {
                let counter_clone = std::sync::Arc::clone(&counter);
                thread_pool
                    .submit(move || {
                        let mut num = counter_clone.lock().unwrap();
                        *num += 1;
                    })
                    .expect("Failed to execute task");
            }
        }

        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 100);
    }

    async fn process_data(data: Vec<tokio::sync::oneshot::Receiver<i32>>) -> Result<i32, Box<dyn std::error::Error>> {

        let mut sum = 0;
        for item in data {
            println!("Waiting for item...");
            let square = item.await?;
            println!("Got: {}", square);
            sum += square;
        }

        Ok(sum)
    }

    #[tokio::test]
    async fn submit_async_task() {
        let pool = ThreadPool::new(4).expect("create pool");

        let mut squares = Vec::new();
        for i in 1..=10 {
            let rx = pool.submit_tokio(move || {
                println!("Computing square of {} on thread#{:?}", i, std::thread::current().id());
                std::thread::sleep(std::time::Duration::from_millis(100 * i as u64));
                i * i
            }).expect("submit failed");
            squares.push(rx);
        }

        let sum_of_squares =process_data(squares).await.expect("process data failed");
        assert_eq!(sum_of_squares, 385);
        println!("Sum of squares: {}", sum_of_squares);
    }
}