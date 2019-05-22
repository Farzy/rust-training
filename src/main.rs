mod some;

fn main() {
    println!("Hello, world!");

    let a: Option<u32> = Some(1);
    let b: Option<u32> = Some(2);
    let c: Option<u32> = None;

    println!("Some(1) + Some(2) = {:?}", some::add_some(a, b));
    println!("Some(1) + None = {:?}", some::add_some(a, c));
}
