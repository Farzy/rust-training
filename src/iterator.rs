pub fn iterators() {
    println!("- Iterator on (1..5)");
    let mut iterator = 1..5;
    while let Some(i) = iterator.next() {
        println!("Iterator: {}", i);
    }

    println!("- Iterator on (4..=10)");
    let mut iterator = 4..=10;
    while let Some(i) = iterator.next() {
        println!("Iterator: {}", i);
    }

    println!("- Iterator with skip and take");
    let iterator = 1..10;
    let mut iterator = iterator.skip(2);
    println!("After skip(2): {:?}", iterator.next());
    let mut taken = iterator.take(2);
    println!("After take(2):");
    while let Some(i) = taken.next() {
        println!("Taken: {}", i);
    }

    println!("- Enumerator");
    let v = vec!["A", "B", "C"];
    let mut e = v.iter().enumerate();
    while let Some(t) = e.next() {
        println!("Enumerator: {:?}", t);
    }

    println!("- Collect");
    let iterator = 1..10;
    let iterator = iterator.skip(2);
    let taken = iterator.take(4);
    let v: Vec<i32> = taken.collect();
    println!("v: {:?}", v);
}

pub fn map_filter_fold() {
    println!("Using map:");
    let iter = 1..10; // Cannot create iterator over range of float64
    let rev: Vec<String> = iter
        .map(|x| { 1.0 / (x as f64) } )
        .map(|x| { x.sin() })
        .map(|x| { format!("{:.6}", x) })
        .collect();
    println!("sin(rev(1..10)) with 6 decimals: {:?}", rev);

    println!("Using filter:");
    let iter = 1..10;
    let evens: Vec<_> = iter.filter(|x| { x % 2 == 0 }).collect();
    println!("Even numbers: {:?}", evens);
}