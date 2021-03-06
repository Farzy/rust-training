pub fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    println!(" - Use u32 cacher");
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    println!(" - Use generic cacher");
    generate_workout_gen(
        "10",
        7.0
    );
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32,u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn generate_workout(intensity: u32, random_number: u32) {
    #[allow(unused_variables)]
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        intensity
    });


    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// ----------------------------------------------------------------------------------------------

use std::hash::Hash;

struct CacherGen<T, U, V>
    where T: Fn(U) -> V
{
    calculation: T,
    value: HashMap<U,V>,
}

impl<T,U,V> CacherGen<T,U,V>
    where T: Fn(U) -> V,
          U: Eq + Hash + Copy,
          V: Copy
{
    fn new(calculation: T) -> CacherGen<T,U,V> {
        CacherGen {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}

fn generate_workout_gen(intensity: &str, random_number: f64) {
    #[allow(unused_variables)]
    let mut expensive_result = CacherGen::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        let i : u64 = intensity.parse().unwrap();
        i
    });


    if intensity < "25" {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3.0 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_different_values_gen() {
        let mut c = CacherGen::new(|a: &str| -> u64 { a.parse().unwrap() });

        let _v1 = c.value("1");
        let v2 = c.value("2");

        assert_eq!(v2, 2);
    }
}
