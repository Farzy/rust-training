// This is just to show that depending of if the "match" is made on an
// enum or a reference to an enum, the matchers placeholder inside the
// enum values are either value or reference to values
#[allow(unused_variables)]
pub fn main() {
    enum EnumMatcher {
        One(u32),
        Two(u8, char),
    }
    use EnumMatcher::*;

    let one = One(1);
    let two = Two(42, 'z');

    match one {
        One(x1)=> println!("x1 is a u32"),
        Two(x1, y1)=> println!("x1 is a u8, y1 is a char"),
    }

    match &two {
        One(x2)=> println!("x2 is a &u32"),
        Two(x2, y2)=> println!("x2 is a &u8, y2 is a &char"),
    }
}
