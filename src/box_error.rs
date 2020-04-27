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

pub fn main() {
    if let Err(e) = run_application() {
        eprintln!("An error happened: {}", e);
    }
}
