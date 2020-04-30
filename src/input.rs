use std::{env, io};

pub fn main() {
    // Only execute this interactive code if "-i" is provided on the command line
    let mut do_input = false;
    for arg in env::args() {
        if arg == "-i" {
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
}