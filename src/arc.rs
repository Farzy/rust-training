// Sample taken from the "arc1.rs" exercise of Rustlings.
// I added the lines commented with ADDON to arc1.

extern crate rand;
extern crate rand_chacha;

use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;
use self::rand::Rng;

fn arc1() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();
    let sumsums = Arc::new(Mutex::new(0)); // ADDON. Does not need "mut"!

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        let child_sumsums = Arc::clone(&sumsums); // ADDON
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
            let mut ss = child_sumsums.lock().unwrap(); // ADDON
            *ss += sum; // ADDON
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
    println!("The sum of sums is: {}", *sumsums.lock().unwrap()); // ADDON
}

fn arc2() {
    let source_sentence = r#"
The platforms we are deploying software on these days are becoming ever more powerful -
modern cloud platforms not only tell us whether our photo shows a puppy or a muffin, they
also compile our code, deploy it, configure the necessary infrastructure, and store our data.

This great convenience and productivity booster also brings a whole new form of lock-in.
Hybrid/multi-cloud setups, which seem to attract many architects' attention these days, are
a good example of the kind of things you'll have to think of when dealing with lock-in. Let's
say you have an application that you'd like to deploy to the cloud. Easy enough to do, but
from an architect's point of view, there are many choices and even more trade-offs,
especially related to lock-in.
    "#;
    let words: Vec<_> = source_sentence.split_whitespace().collect();
    let shared_words = Arc::new(words);
    let sentence = Arc::new(Mutex::new(String::new()));
    let mut joinhandles = Vec::new();

    for i in 0..30 {
        let child_words = Arc::clone(&shared_words);
        let child_sentence = Arc::clone(&sentence);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for count in 0..rng.gen_range(1, 10) {
                thread::sleep(Duration::from_secs_f64(rng.gen_range(0.001, 0.5)));
                let x = rng.gen_range(0, child_words.len());
                let w = child_words[x];
                print!("Thread {}, word #{}: {}. ", i, count+1, w);
                let mut s = child_sentence.lock().unwrap();
                (*s).push_str(" ");
                (*s).push_str(w);
            }
        });
        joinhandles.push(handle);
    }

    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
    println!("\n");
    let final_sentence = String::from(&*sentence.lock().unwrap());
    println!("Random sentence: {}", final_sentence.trim());
}

pub fn main() {
    println!(" - arc1:");
    arc1();
    println!(" - arc2:");
    arc2();
}