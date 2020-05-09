pub fn main() {
    let mut num = 5;

    println!("num = {}", num);

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 = {:?}, r2 = {:?}", r1, r2);

    unsafe {
        *r2 = 2;
    }

    println!("num after unsafe change = {}", num);
}
