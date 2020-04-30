use std::env;

pub fn main() {
    let env_vars = env::vars();
    for (key,val) in env_vars {
        println!("{}: {}", key, val);
    }

}
