#[macro_use] extern crate quick_error;
#[macro_use] extern crate error_chain;
extern crate failure;
extern crate failure_derive;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use std::env;
use rust_training::helper;

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
mod functions;
mod arc;
mod smart_pointers;
mod mock;
mod reference_cycle;

// Simplify long hashmap type
type FunctionHash = HashMap<String, (String, fn())>;

fn usage(functions: &FunctionHash) {
    eprintln!(r#"
Usage: PROGNAME [options] [function...]

 -h: Print this help
 -i: Only use by "do_input" for interaction

Specify one or more function names.
If no function name is giver then all of them are executed.

List of functions:"#);
    for (name, (description, _)) in functions.iter() {
        eprintln!(" - {}: {}", name, description);
    }
}

fn main() {
    let mut functions: FunctionHash = HashMap::new();
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
    functions.insert(String::from("function-crate"), (String::from("Functions and Crates"), functions::main));
    functions.insert(String::from("old-tutorial"), (String::from("Old tutorial code"), samples::main));
    functions.insert(String::from("arc"), (String::from("Asynchronous RC"), arc::main));
    functions.insert(String::from("smart-pointers"), (String::from("Smart Pointers"), smart_pointers::main));
    functions.insert(String::from("mock"), (String::from("Mock using RefCell"), mock::main));
    functions.insert(String::from("ref-cycle"), (String::from("Reference Cycle"), reference_cycle::main));

    if env::args().len() == 1 { // No arguments
        for (name, (description, func)) in functions.iter() {
            helper::section(&format!("{} ({})", description, name));
            func();
        }
    } else {
        for k in env::args().skip(1) {
            if k == "-h" || k == "--help" {
                usage(&functions);
                return;
            } else if functions.contains_key(&k) {
                let (description, func) = functions.get(&k).unwrap();
                helper::section(description);
                func();
            } else {
                eprintln!("\nERROR: Function '{}' not found", k);
            }
        }
    }
}
