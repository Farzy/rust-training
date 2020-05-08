use rust_training::helper;
use std::sync::Mutex;

fn simple_mutex() {
    let m = Mutex::new(5);
    println!("m before block = {:?}", m);

    {
        let mut num = Mutex::lock(&m).unwrap();
        *num = 6;
    }

    println!("m after block = {:?}", m);
}

pub fn main() {
    helper::subsection("Simple Mutex");
    simple_mutex();
}
