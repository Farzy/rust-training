use std::fs::{self, File};
use std::io::{self, ErrorKind};

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

pub fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}
