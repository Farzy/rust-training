use rust_training::helper;
use std::sync::{Mutex, Arc};
use std::thread;

fn simple_mutex() {
    let m = Mutex::new(5);
    println!("m before block = {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m after block = {:?}", m);
}

fn mutex_in_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
           let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn main() {
    helper::subsection("Simple Mutex");
    simple_mutex();

    helper::subsection( "Mutex in thread");
    mutex_in_threads();
}
