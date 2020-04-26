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

    *list.first_mut().expect("list was empty") += 1;

    let list_first = list.first();
    let list_last = list.last();

    println!(
        "The first element is {:?} and the last is {:?}",
        list_first,
        list_last
    )
}