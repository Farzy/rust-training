use rust_training::helper;

pub fn main() {
    let mut v = Vec::new();

    v.push('x');
    v.push('y');
    v.push('z');

    helper::subsection("Iterations on Vec whose content has Copy");
    println!("With for:");
    for item in v {
        println!("for1: {}", item);
    }
    println!("At this stage 'v' has been consumed by the 'for' iterator.");
    // Try this
    // for item in v {
    //     println!("for1: {}", item);
    // }

    let mut v = Vec::new();

    v.push('x');
    v.push('y');
    v.push('z');

    helper::subsection("Iterations on &Vec whose content has Copy");
    println!("With for:");
    for item in &v {
        println!("for1 &v: {}", item);
    }
    println!("\nTwice:");
    for item in &v {
        println!("for2 &v: {}", item);
    }


    helper::subsection("Iterations on Vec whose content is not Copy");
    let v2 = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];
    println!("Creating a first iter() on v2:");
    let mut v2iter1 = v2.iter();
    while let Some(s) = v2iter1.next() {
        println!("v2 iter 1: {}", s);
    }
    println!("\nCreating a second iter() on v2:");
    let mut v2iter2 = v2.iter();
    while let Some(s) = v2iter2.next() {
        println!("v2 iter 2: {}", s);
    }
    println!("\nNow for on v2 (no &):");
    for s in v2 {
        println!("for v2: {}", s);
    }
    println!("At this stage v2 has been consumed by the for iterator.");

    let v2 = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];
    println!("\nNow for on &v2:");
    for s in &v2 {
        println!("for v2: {}", s);
    }
    println!("\nAgain, for on &v2:");
    for s in &v2 {
        println!("for v2: {}", s);
    }
}
