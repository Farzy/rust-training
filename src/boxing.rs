#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

pub fn main() {
    let b = Box::new(5);
    println!("Box<u32> = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List = {:?}", list);
}
