use std::fs::File;
use std::io::ErrorKind;

pub fn main() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create a file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

//    let _f = File::open("hello2.txt").expect("Failed to open hello.txt");
}
