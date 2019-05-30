use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::iter::FromIterator;

pub fn main() {
    let mut nums = [0u8; 10];

    thread_rng().fill(&mut nums);

    println!("nums = {:?}", nums);

    nums.shuffle(&mut thread_rng());
    println!("shuffled nums = {:?}", nums);

    let l = ['a', 'b', 'c'];
//    let s : String = l.iter().collect();
    let s : String = String::from_iter(&l);
    println!("s = {}", s);
    println!("l = {:?}", l);
}
