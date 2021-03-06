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

// XXX Using i32::MIN as "None" value arbitrarily
impl Default for Node {
    fn default() -> Self {
        Node {
            value: i32::MIN,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value != i32::MIN { // Our arbitrary "None" value
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

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("leaf parent = {}", leaf.parent.borrow().upgrade().unwrap_or_default());

    {
        println!("\nCreate branch in inner scope\n");
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        println!("branch before strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {}", *leaf.parent.borrow().upgrade().unwrap_or_default());
        println!("leaf = {}", leaf);
        println!("branch after strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        println!("\nLeave inner scope\n");
    }

    println!("leaf parent = {}", *leaf.parent.borrow().upgrade().unwrap_or_default());
    println!("leaf = {}", leaf);
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

pub fn main() {
    helper::subsection("Reference cycle");
    reference_cycle();

    helper::subsection("Using Weak Ref");
    weak_ref();
}
