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

    let v = vec!['a', 'b', 'c', 'd'];
    println!("v: {:?}", v);
    let s: String = v.into_iter().collect();
    println!("v into s: {}", s);
//    println!("v after into_iter: {:?}", v); // v is moved by into_iter

    let vecs = nums.to_vec();
    println!("nums = {:?}", nums); // no move
    let s  = match String::from_utf8(vecs) {
      Ok(x) => x.to_string(),
      Err(e)=> e.to_string()
    };
    println!("Stringification of '{:?}' = {}", nums, s);
}
