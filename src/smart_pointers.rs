use std::ops::Deref;
use rust_training::helper;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

fn boxing() {
    let b = Box::new(5);
    println!("Box<u32> = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List = {:?}", list);
}

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

fn deref() {
    let x = 5;
    let y = MyBox::new(x);
    println!("Asserting that *MyBox::new(x) == x");
    assert_eq!(5, x);
    assert_eq!(x, *y);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

#[allow(unused_variables)]
fn dropper() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    let e = CustomSmartPointer {
        data: String::from("explicite dropper")
    };
    println!("CustomSmartPointers created.");
    println!("About to drop CSP with data '{}' early:", e.data);
    drop(e);
    // Next line won't compile
    // println!("At this point '{}' is dropped.", e.data);
    println!("At this point it is dropped.");
}

pub fn main() {
    helper::subsection("Box type");
    boxing();

    helper::subsection("Deref trait");
    deref();

    helper::subsection("Drop trait");
    dropper();
}
