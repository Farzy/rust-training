mod hello;
use self::hello::hello;
// This would also work:
// use crate::functions::hello::hello;

fn say_hello(name: &str) {
    println!("Hello {}!", name)
}

fn add(a: i32, b: i64) -> i32 {
    return a + (b as i32);
}

pub fn main() {
    say_hello("world");

    let i1 = 1;
    let i2 = 2;
    println!("{} + {} = {}", i1, i2, add(i1, i2));

    hello();
}
