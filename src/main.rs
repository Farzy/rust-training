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

use std::collections::HashMap;
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
mod fibonacci;
mod envvars;
mod vector;
mod linkedlist;

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

    hello();

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
    functions.insert(String::from("map-filter-fold"), (String::from("Map, Filter, Fold…"), iterator::main));
    functions.insert(String::from("iterators"), (String::from("Iterators"), iterator::iterators));
    functions.insert(String::from("dynamic"), (String::from("Dynamic dispath"), dynamic::main));
    functions.insert(String::from("traits"), (String::from("Traits"), traits::main));
    functions.insert(String::from("ownership"), (String::from("Ownership"), ownership::main));
    functions.insert(String::from("substrings"), (String::from("Substrings"), strings::main));
    functions.insert(String::from("fibonacci"), (String::from("Fibonacci"), fibonacci::main));
    functions.insert(String::from("envvars"), (String::from("Environment variables"), envvars::main));
    functions.insert(String::from("vector"), (String::from("Vector"), vector::main));
    functions.insert(String::from("linked-list"), (String::from("LinkedList"), linkedlist::main));
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
