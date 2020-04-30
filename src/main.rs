#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate error_chain;

extern crate failure;
extern crate failure_derive;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::collections::{LinkedList, HashMap};
use std::env;
mod hello;
use crate::hello::hello;
mod strings;
mod ownership;
mod dynamic;
mod traits;
mod iterator;
mod input;
mod myrand;
mod drop;
// This module contains all the functions that we previously in my "samples" Rust project
mod samples;
mod box_error;
mod custom_error;
mod quick_error_test;
mod error_chain_test;
mod failure_test;
mod json;
mod result_option;
mod lifetime;
mod generics;

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

fn usage(functions: &HashMap<String, (String, fn())>) {
    eprintln!(r#"
Usage: PROGNAME [options] [function]

 -h: Print this help
 -i: Only use by "do_input" for interaction

If no function name is giver than all are executed.

List of functions:"#);
    for (name, (description, _)) in functions.iter() {
        eprintln!(" - {}: {}", name, description);
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
    iterator::main();

    let mut functions: HashMap<String, (String, fn())> = HashMap::new();
    functions.insert(String::from("cust-err"), (String::from("Custom error"), custom_error::main));
    functions.insert(String::from("quick-err"), (String::from("Quick error"), quick_error_test::main));
    functions.insert(String::from("error-chain"), (String::from("Error chain"), error_chain_test::main));
    functions.insert(String::from("failure"), (String::from("Failure"), failure_test::main));
    functions.insert(String::from("json"), (String::from("JSON"), json::main));
    functions.insert(String::from("result-option"), (String::from("Result<> and Option"), result_option::main));
    functions.insert(String::from("lifetime"), (String::from("Lifetime"), lifetime::main));
    functions.insert(String::from("generics"), (String::from("Generics"), generics::main));
    functions.insert(String::from("drop"), (String::from("Drop"), drop::main));
    functions.insert(String::from("box-error"), (String::from("Box Error"), box_error::main));
    functions.insert(String::from("randomness"), (String::from("Randomness"), myrand::main));
    functions.insert(String::from("input"), (String::from("Input"), input::main));
    // functions.insert(String::from(""), (String::from(""), ::main));

    if env::args().len() == 1 {
        for (name, (description, func)) in functions.iter() {
            section(&format!("{} ({})", description, name));
            func();
        }
    } else {
        let k = env::args().skip(1).next().unwrap();
        if k == "-h" || k == "--help" {
            usage(&functions);
        } else if functions.contains_key(&k) {
            let (description, func) = functions.get(&k).unwrap();
            section(description);
            func();
        } else {
            eprintln!("ERROR: Function '{}' not found", k);
        }
    }
}
