use std::num::ParseIntError;

pub fn main() {
    println!("Convert Result<> to Option:");
    let r1 = "42".parse::<i32>();
    let r2 = "toto".parse::<i32>();
    println!("Results: '{:?}' / '{:?}", r1, r2);
    let o1 = r1.ok();
    let o2 = r2.ok();
    println!("Options: {:?} / {:?}'", o1, o2);

    println!("\nConvert Option to Result<>:");
    let o1: Option<i32> = Some(42);
    let o2: Option<i32> = None;
    println!("Options: {:?} / {:?}", o1, o2);
    let r1 = o1.ok_or("Not a number");
    let r2 = o2.ok_or("Not a number");
    println!("Results: '{:?}' / '{:?}'", r1, r2);

    println!("\nDefault values on Result<>:");
    let r1 = "42".parse::<i32>();
    let r2 = "toto".parse::<i32>();
    println!("Results: '{:?}' / '{:?}", r1, r2);
    let i1 = r1.unwrap_or_default();
    let i2 = r2.unwrap_or_default();
    println!("unwrap_or_default: {:?} / {:?}'", i1, i2);

    println!("\nDefault values on Option:");
    let o1: Option<i32> = Some(42);
    let o2: Option<i32> = None;
    println!("Options: {:?} / {:?}", o1, o2);
    let i1 = o1.unwrap_or_default();
    let i2 = o2.unwrap_or_default();
    println!("Results: '{:?}' / '{:?}'", i1, i2);

    println!("\nMap over Option:");
    let a = vec![Some(2), Some(-3), None, Some(10)];
    println!("Array of Option: {:?}", a);
    let b: Vec<Option<i32>> = a.iter().map(|x| x.map(|y| y*y)).collect();
    println!("Array of Option squared: {:?}", b);

    println!("\nMap over Result<>:");
    let s = vec!["2", "32", "toto", "-10"];
    println!("Array of strings: {:?}", s);
    let numbers: Vec<Result<i32, ParseIntError>> = s.iter().map(|x| x.parse::<i32>()).collect();
    println!("Array of Result<i32>: {:?}", numbers);
}
