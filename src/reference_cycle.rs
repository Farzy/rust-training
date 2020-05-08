use rust_training::helper;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::*;

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn reference_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a = {:?}", a);
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    println!("\n### Uncomment the line at the end of this function to overflow the stack! ###");
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            value: -9999,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value != -9999 {
            write!(
                f,
                "Node(uid: {}, parent: {}, children: {})",
                self.value,
                self.parent.borrow().upgrade().unwrap_or_default(),
                self.children.borrow().len()
            )
        } else {
            write!(f, "Node(None)")
        }
    }
}

fn weak_ref() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf parent = {}",
        leaf.parent.borrow().upgrade().unwrap_or_default()
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "leaf parent = {}",
        *leaf.parent.borrow().upgrade().unwrap_or_default()
    );
    println!(
        "leaf = {}", leaf);
}

pub fn main() {
    helper::subsection("Reference cycle");
    reference_cycle();

    helper::subsection("Using Weak Ref");
    weak_ref();
}
