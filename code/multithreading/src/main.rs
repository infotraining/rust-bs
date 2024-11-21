use std::sync::mpsc;
use std::thread;

mod thread_pool;

fn simple_multithreading() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(500));
    }

    handle.join().unwrap();
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 1..5 {
            let val = format!("Hi -  {}", i);
            tx1.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for i in 1..10 {
            let val = format!("Hello -  {}", i);
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_millis(250));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn thread_pool() {
    let pool = thread_pool::ThreadPool::new(4);

    for i in 0..10 {
        pool.execute(move || {
            println!("Task {}", i);
        });
    }
}

fn main() {
    message_passing();

    println!("-----------------");

    simple_multithreading();

    println!("-----------------");

    thread_pool();
}
