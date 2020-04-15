mod hello;
use std::collections::LinkedList;
use std::env;
use crate::hello::hello;

fn say_hello(name: &str) {
    println!("Hello {}!", name)
}

fn add(a: i32, b: i64) -> i32 {
    return a + (b as i32);
}

fn section(title: &str) {
    let len = title.len();
    let mut dashes = String::new();
    for _ in 0..len {
        dashes.push_str("-");
    }
    println!("\n+-{}-+", dashes);
    println!("| {} |", title);
    println!("+-{}-+\n", dashes);
}

fn main() {
    section("Functions");
    say_hello("world");

    let i1 = 1;
    let i2 = 2;
    println!("{} + {} = {}", i1, i2, add(i1, i2));

    section("LinkedList");
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

    section("Vector");
    let mut v = Vec::new();

    v.push('x');
    v.push('y');
    v.push('z');

    for item in v {
        println!("{}", item);
    }

    hello();

    section("Env vars");

    let env_vars = env::vars();
    for (key,val) in env_vars {
        println!("{}: {}", key, val);
    }
}
