use rust_training::helper;

mod read;

pub fn main() {
    helper::subsection("Trait Read");
    read::main().unwrap_or_else(|err| println!("Read error: {}", err));
}
