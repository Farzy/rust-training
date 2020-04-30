use std::collections::HashMap;

const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

fn fib(n: u64) -> u64 {
    if n == 0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = fib_dyn(n-1, map) + fib_dyn(n-2, map);
                map.insert(n, val);
                val
            }
        }
    }
}

pub fn main() {
    println!(" - Fibonacci recursive");
// Cannot reach 50 in a minute
    for n in &[0, 1, 5, 10, 15, 20, 30, 35, 40] {
        println!("fib({}) = {}", *n, fib(*n));
    }

    println!(" - Fibonacci dynamic");
    let mut map = HashMap::new();
    for n in &[0, 1, 5, 10, 15, 20, 30, 35, 40] {
        println!("fib({}) = {}", *n, fib_dyn(*n, &mut map));
    }
}
