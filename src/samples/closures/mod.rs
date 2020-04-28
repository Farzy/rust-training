pub fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
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

#[allow(unused_variables)]
fn generate_workout(intensity: u32, random_number: u32) {
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

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

// ----------------------------------------------------------------------------------------------

use std::hash::Hash;

#[allow(dead_code)]
struct CacherGen<T, U, V>
    where T: Fn(U) -> V
{
    calculation: T,
    value: HashMap<U,V>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[allow(unused_variables)]
fn generate_workout_gen(intensity: &str, random_number: f64) {
    let mut expensive_result = CacherGen::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        let i : u64 = intensity.parse().unwrap();
        i
    });


    if intensity == "25" {
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

#[test]
fn call_with_different_values_gen() {
    let mut c = CacherGen::new(|a : &str| -> u64 { a.parse().unwrap() });

    let _v1 = c.value("1");
    let v2 = c.value("2");

    assert_eq!(v2, 2);
}
