mod some;
mod vec;
mod strings;
mod hashmaps;
mod rands;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args = {:?}", args);

    if args.len() > 1 && args[1] =="panic" {
        panic!("Panic argument provided");
    }

    println!("Hello, world!");

    banner("Option");
    let a: Option<u32> = Some(1);
    let b: Option<u32> = Some(2);
    let c: Option<u32> = None;

    println!("Some(1) + Some(2) = {:?}", some::add_some(a, b));
    println!("Some(1) + None = {:?}", some::add_some(a, c));

    banner("Vectors");
    vec::vecs();

    banner("Strings");
    strings::main();

    banner("Hash maps");
    hashmaps::main();

    banner("Random");
    rands::main();
}

fn banner(s: &str) {
    println!("\n\t===================");
    println!("\t\t{}", s);
    println!("\t===================\n");
}
