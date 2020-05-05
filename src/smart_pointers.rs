use std::ops::Deref;
use rust_training::helper;

// --------------------------------------------------------------------------------------------

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

// --------------------------------------------------------------------------------------------

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

// --------------------------------------------------------------------------------------------

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

// --------------------------------------------------------------------------------------------

use std::rc::Rc;

#[derive(Debug)]
enum ListRc<T> {
    ConsRc(T, Rc<ListRc<T>>),
    NilRc,
}

use self::ListRc::*;

fn reference_counter() {
    // This won't compile
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(3, Box::new(a));

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("a = {:?}", a);
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    println!("b = {:?}", b);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsRc(3, Rc::clone(&a));
        println!("c = {:?}", c);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("Dropping b");
    drop(b);
    println!("count after b is dropped = {}", Rc::strong_count(&a));

}

// --------------------------------------------------------------------------------------------

pub fn main() {
    helper::subsection("Box type");
    boxing();

    helper::subsection("Deref trait");
    deref();

    helper::subsection("Drop trait");
    dropper();

    helper::subsection("RC type");
    reference_counter();
}
