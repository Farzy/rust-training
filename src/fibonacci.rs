pub fn main() {
    println!("fib(10) =");
    for (i, n) in fib(100).iter().enumerate() {
        println!("{: >3}: {: >25}", i, n);
    }
}

fn fib(n: usize) -> Vec<i128> {
    let mut f : Vec<i128> = vec![0, 1];

    for i in 2..=n {
        f.push(f[i-2] + f[i-1]);
    }

    f
}
