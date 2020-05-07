// This is just to show that depending of if the "match" is made on an
// enum or a reference to an enum, the matchers placeholder inside the
// enum values are either value or reference to values
pub fn main() {
    enum EnumMatcher {
        One(u32),
        Two(u8, char),
    }
    use EnumMatcher::*;

    let one = One(1);
    let two = Two(42, 'z');

    match one {
        One(_x1)=> println!("_x1 is a u32"),
        Two(_x1, _y1)=> println!("_x1 is a u8, _y1 is a char"),
    }

    match &two {
        One(_x2)=> println!("_x2 is a &u32"),
        Two(_x2, _y2)=> println!("_x2 is a &u8, _y2 is a &char"),
    }
}
