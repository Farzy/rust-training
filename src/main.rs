mod hello;
use std::collections::{LinkedList, HashMap};
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

const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

fn fib(n: u64) -> u64 {
    if n == 0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = fib_dyn(n-1, map) + fib_dyn(n-2, map);
                map.insert(n, val);
                val
            }
        }
    }
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

    section("Fibonacci recursive");

    // Cannot reach 50 in a minute
    for n in &[0, 1, 5, 10, 15, 20, 30, 35, 40, 42, 45] {
        println!("fib({}) = {}", *n, fib(*n));
    }

    section("Fibonacci dynamic");

    let mut map = HashMap::new();
    for n in &[0, 1, 5, 10, 15, 20, 30, 35, 40, 42, 45] {
        println!("fib({}) = {}", *n, fib_dyn(*n, &mut map));
    }
}
