mod some;
mod vec;
mod strings;
mod hashmaps;

fn main() {
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
}

fn banner(s: &str) {
    println!("\n\t===================");
    println!("\t\t{}", s);
    println!("\t===================\n");
}
