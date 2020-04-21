pub fn strings() {
    let str = String::from("Hello world!");

    let first = match str.chars().next() {
        Some(x) => x.to_lowercase().collect::<String>(),
        None => String::new()
    };

    println!("first: {:?}", first);
}
