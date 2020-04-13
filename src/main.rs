use std::collections::LinkedList;
use crate::hello::hello;

fn say_hello(name: &str) {
    println!("Hello {}!", name)
}

fn add(a: i32, b: i64) -> i32 {
    return a + (b as i32);
}

mod hello {
    pub fn hello() {
        println!("Hello, world!");
    }
}

fn main() {
    println!("- Functions");
    say_hello("world");

    let i1 = 1;
    let i2 = 2;
    println!("{} + {} = {}", i1, i2, add(i1, i2));

    println!("- LinkedList");
    let mut ll = LinkedList::new();

    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(4);

    println!("Print LL using debug syntax:");
    println!("ll = {:?}", ll);
    println!("Print LL using loop:");
    for foo in ll {
        println!("{}", foo);
    }

    println!("- Vector");
    let mut v = Vec::new();

    v.push('x');
    v.push('y');
    v.push('z');

    for item in v {
        println!("{}", item);
    }

    hello();
}
