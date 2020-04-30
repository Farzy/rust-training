pub fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += value;
    }
    sum
}

pub fn borrow_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += *value;
    }
    sum
}

pub fn cap_values_owned(max: i32, mut v: Vec<i32>) -> Vec<i32> {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
    v
}
pub fn cap_values(max: i32, v: &mut Vec<i32>) {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
}

pub fn mutable_and_immutable_borrows() {
    let mut list = vec![1, 2, 3];

    // Mutable borrow acceptable here because its scope is only this line
    *list.first_mut().expect("list was empty") += 1;

    let list_first = list.first();
    let list_last = list.last();

    println!(
        "The first element is {:?} and the last is {:?}",
        list_first,
        list_last
    );

    // Works also here because the scope of list_first and list_last has ended.
    // This did not compile in previous versions of the compiler (~ 1.24).
    *list.first_mut().expect("list was empty") += 1;
}

pub fn main() {
    let values = vec![1, 2, 3, 4, 5];
    let sum = take_ownership_sum(values);
    println!("Sum = {}", sum);
    // println!("Sum of {} values: {}", values.len(), sum); // Forbidden

    let values2 = vec![1, 2, 3, 4, 5];
    let sum = borrow_sum(&values2);
    println!("Sum of {} values: {}", values2.len(), sum);

    println!("cap_values_owned");
    let mut values = vec![1, 2, 3, 10000, 5];
    values = cap_values_owned(10, values);

    for v in &values {
        println!("Capped value owned: {}", *v);
    }

    for v in values {
        println!("Capped value owned: {}", v);
    }

    println!("cap_values mutable arg");
    let mut values = vec![1, 2, 3, 10000, 5];
    cap_values(10, &mut values);

    for v in &values {
        println!("Capped value mut: {}", *v);
    }

    mutable_and_immutable_borrows();
}
