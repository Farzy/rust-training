use rust_training::helper;
use std::slice;

static mut COUNTER: u32 = 0;

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

    helper::subsection("Safe abstractions over unsafe code");

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    println!("slice_at_mut()");
    let (a, b) = r.split_at_mut(3);

    assert_eq!(&[1, 2, 3], a);
    assert_eq!(&[4, 5, 6], b);

    println!("Reproduce slide_at_mut() for &[i32]");
    let (a, b) = split_at_mut(r,3);

    assert_eq!(&[1, 2, 3], a);
    assert_eq!(&[4, 5, 6], b);

    println!("Create an invalid slice");
    let address = 0x01234usize;
    let r = address as *mut i32;

    // This line is still safe
    let _slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // This one will crash with an error like:
    //   Process finished with exit code 139 (interrupted by signal 11: SIGSEGV)
    // or:
    //   Segmentation fault: 11
    //println!("slice = {:?}", slice);

    helper::subsection("Call external code");

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    helper::subsection("Static variables");

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// error[E0499]: cannot borrow `*slice` as mutable more than once at a time
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
//
//     (&mut slice[..mid], &mut slice[mid..])
// }

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
