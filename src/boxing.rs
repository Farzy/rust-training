use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn main() {
    let b = Box::new(5);
    println!("Box<u32> = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List = {:?}", list);

    let x = 5;
    let y = MyBox::new(x);
    println!("Asserting that *MyBox::new(x) == x");
    assert_eq!(5, x);
    assert_eq!(x, *y);
}