use std::collections::{LinkedList, HashMap};
use std::{env, io};
mod hello;
use crate::hello::hello;
mod strings;
mod ownership;
mod dynamic;
mod traits;
mod iterator;
mod myrand;
mod drop;
// This module contains all the functions that we previously in my "samples" Rust project
mod samples;

fn say_hello(name: &str) {
    println!("Hello {}!", name)
}

fn add(a: i32, b: i64) -> i32 {
    return a + (b as i32);
}

fn section(title: &str) {
    let len = title.len();
    let dashes = "-".repeat(len);
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
    // Execute older code
    section("Old samples");
    samples::main();

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
    for n in &[0, 1, 5, 10, 15, 20, 30, 35, 40] {
        println!("fib({}) = {}", *n, fib(*n));
    }

    section("Fibonacci dynamic");
    let mut map = HashMap::new();
    for n in &[0, 1, 5, 10, 15, 20, 30, 35, 40] {
        println!("fib({}) = {}", *n, fib_dyn(*n, &mut map));
    }

    section("Substrings");
    strings::main();

    section("Ownership");
    let values = vec![1, 2, 3, 4, 5];
    let sum = ownership::take_ownership_sum(values);
    println!("Sum = {}", sum);
    // println!("Sum of {} values: {}", values.len(), sum); // Forbidden

    let values2 = vec![1, 2, 3, 4, 5];
    let sum = ownership::borrow_sum(&values2);
    println!("Sum of {} values: {}", values2.len(), sum);

    println!("cap_values_owned");
    let mut values = vec![1, 2, 3, 10000, 5];
    values = ownership::cap_values_owned(10, values);

    for v in &values {
        println!("Capped value owned: {}", *v);
    }

    for v in values {
        println!("Capped value owned: {}", v);
    }

    println!("cap_values mutable arg");
    let mut values = vec![1, 2, 3, 10000, 5];
    ownership::cap_values(10, &mut values);

    for v in &values {
        println!("Capped value mut: {}", *v);
    }

    ownership::mutable_and_immutable_borrows();

    section("Traits");
    traits::main();

    section("Dynamic dispatch");
    dynamic::main();

    section("Iterators");
    iterator::iterators();

    section("Map, Filter, Fold…");
    iterator::map_filter_fold();

    section("Randomness");
    myrand::main();

    section("Input");
    // Only execute this interactive code if "-i" is provided on the command line
    let mut do_input = false;
    for arg in env::args() {
        if arg == "-i".to_string() {
            do_input = true;
            break;
        }
    }
    if do_input {
        let mut word = String::new();
        while word.trim() != "rust" {
            println!("What is the secret word?");
            word.clear();
            io::stdin().read_line(&mut word).expect("Could not read stdin");
        }
        println!("You found the secret word! Please proceed!");
    } else {
        println!("SKIPPED: In order to execute this section add '-i' to the command line.")
    }

    section("Drop");
    drop::main();
}
