use rust_training::helper;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

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

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("hi");
        println!("T: Sending message…");
        tx.send(val).unwrap();
        // Try this
        // println!("Sent values: {}", val);
        println!("T: Sleeping for 500ms…");
        thread::sleep(Duration::from_millis(500));
        println!("T: Done sleeping in thread");
    });

    let received = rx.recv().unwrap();
    println!("M: Got: {}", received);

    handle.join().unwrap();
}

fn message_passing_early_close_sender() {
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    let handle = thread::spawn(move || {
        println!("T: Sending no message!");
        drop(tx);
        println!("T: Sleeping for 500ms…");
        thread::sleep(Duration::from_millis(500));
        println!("T: Done sleeping in thread");
    });

    let received = rx.recv().unwrap_err();
    println!("M: Got an error as expected: {}", received);

    handle.join().unwrap();
}

fn message_passing_sleep() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("M: Got: {}", received);
    }

    handle.join().unwrap();
}

fn message_passing_multiple_senders() {
    let (tx, rx) = mpsc::channel();

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 1..5 {
        let tx1 = tx.clone();
        let handle = thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx1.send(format!("T{}: {}", i, val)).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });
        handles.push(handle);
    }

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(format!("T0: {}", val)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("M: Got: {}", received);
    }

    for handle in handles {
        handle.join().unwrap();
    }
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

    helper::subsection("Thread with message passing");
    message_passing();

    helper::subsection("Thread with message passing, no incoming message");
    message_passing_early_close_sender();

    helper::subsection("Thread with multiple messages passing and pauses");
    message_passing_sleep();

    helper::subsection("Thread with multiple senders");
    message_passing_multiple_senders();
}
