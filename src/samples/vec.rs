pub fn vecs() {
    #![allow(unused_variables)]
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let w = vec![1, 2, 3, 4];

        // do stuff with w
        println!("W = {:?}", w);

    } // <- w goes out of scope and is freed here
//    println!("W = {}", w);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    match v.get(6) {
        Some(sixth) => println!("The sixth element is {}", sixth),
        None => println!("There is no sixth element."),
    }

    for i in &v {
        println!("{}", i);
    }

    let mut w = v;
    for i in  &mut w {
        *i += 10;
        println!("{}", i);
    }
}
