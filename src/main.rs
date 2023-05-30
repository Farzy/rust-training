#[macro_use] extern crate quick_error;
#[macro_use] extern crate error_chain;
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
mod match_ref;
mod threads;
mod mutex;
mod oop;
mod unsafe_rust;
mod advanced_traits;
mod macros;
mod monty_hall;
mod io;

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
    for (name, (description, _)) in functions {
        eprintln!(" - {}: {}", name, description);
    }
}

fn main() {
    let mut functions: FunctionHash = HashMap::new();

    macro_rules! module {
        ($mod_name:ident, $desc:expr) => (
            functions.insert(String::from(stringify!($mod_name)),
                             (String::from($desc), $mod_name::main));
        );
    }

    module!(custom_error, "Custom error");
    module!(quick_error_test, "Quick error");
    module!(error_chain_test, "Error chain");
    module!(failure_test, "Failure");
    module!(json, "JSON");
    module!(result_option, "Result<> and Option");
    module!(lifetime, "Lifetime");
    module!(generics, "Generics");
    module!(drop, "Drop");
    module!(box_error, "Box Error");
    module!(myrand, "Randomness");
    module!(input, "Input");
    module!(iterator, "Map, Filter, Fold…");
    module!(iterator, "Iterators");
    module!(dynamic, "Dynamic dispath");
    module!(traits, "Traits");
    module!(ownership, "Ownership");
    module!(strings, "Substrings");
    module!(fibonacci, "Fibonacci");
    module!(envvars, "Environment variables");
    module!(vector, "Vector");
    module!(linkedlist, "LinkedList");
    module!(functions, "Functions and Crates");
    module!(samples, "Old tutorial code");
    module!(arc, "Asynchronous RC");
    module!(smart_pointers, "Smart Pointers");
    module!(mock, "Mock using RefCell");
    module!(reference_cycle, "Reference Cycle");
    module!(match_ref, "Match & references");
    module!(threads, "Threads");
    module!(mutex, "Mutex");
    module!(oop, "Object-oriented programming");
    module!(unsafe_rust, "Unsafe Rust");
    module!(advanced_traits, "Advanced Traits");
    module!(macros, "Macros");
    module!(monty_hall, "Monty Hall");
    module!(io, "io Read/Write");

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
