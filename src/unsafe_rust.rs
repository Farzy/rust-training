use rust_training::helper;

pub fn main() {
    helper::subsection("Raw pointers");
    let mut num = 5;

    println!("num = {}", num);

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 = {:?}, r2 = {:?}", r1, r2);

    unsafe {
        println!("*r1 = {}, *r2 = {}", *r1, *r2);
        *r2 = 2;
    }

    println!("num after unsafe change = {}", num);

    unsafe {
        println!("*r1 = {}, *r2 = {}", *r1, *r2);
    }

    helper::subsection("Calling unsafe functions");

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

}
