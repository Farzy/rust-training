use rust_training::helper;
use std::thread;
use std::time::Duration;

fn thread_no_join() {
    thread::spawn(|| {
       for i in 1..10 {
           println!("hi number {} from the spawned thread!", i);
           thread::sleep(Duration::from_millis(1));
       }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_join() {
    let handle = thread::spawn(|| {
       for i in 1..10 {
           println!("hi number {} from the spawned thread!", i);
           thread::sleep(Duration::from_millis(1));
       }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn thread_join_early() {
    let handle = thread::spawn(|| {
       for i in 1..10 {
           println!("hi number {} from the spawned thread!", i);
           thread::sleep(Duration::from_millis(1));
       }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_move() {
    let v = vec![1, 2, 3];

    // Try removing "move" and check the error message at build time
    let handle = thread::spawn(move || {
       println!("Here is a vector: {:?}", v);
    });

    // Try this
    // drop(v);

    handle.join().unwrap();
}

pub fn main() {
    helper::subsection("Thread with no join");
    thread_no_join();

    helper::subsection("Thread with join");
    thread_join();

    helper::subsection("Thread with early join");
    thread_join_early();

    helper::subsection("Thread with move in closure");
    thread_move();
}
