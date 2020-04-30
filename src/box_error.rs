use std::env;
use std::error::Error;

fn num_threads() -> Result<usize, Box<dyn Error>> {
    let s = env::var("NUM_THREADS")?;
    let n: usize = s.parse()?;
    Ok(n + 1)
}

fn run_application() -> Result<(), Box<dyn Error>> {
    let num = num_threads()?;
    println!("The number of threads is {}", num);
    // Rest of program functionality
    Ok(())
}

fn box_error() {
    if let Err(e) = run_application() {
        eprintln!("An error happened: {}", e);
    }
}

pub fn main() {
    let envvar = "NUM_THREADS";
    println!("Testing with no environment variable named {}…", envvar);
    box_error();
    println!("Testing with non numeric env. var. 'foo'…");
    env::set_var(envvar, "foo");
    box_error();
    println!("Testing with numeric env. var. '42'…");
    env::set_var(envvar, "42");
    box_error();
}
