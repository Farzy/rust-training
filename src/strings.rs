#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial strings");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 = {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
//    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    let s = String::from("Hola");
    println!("len {} = {}", s, s.len());
    let s = String::from("Здравствуйте");
    println!("len {} = {}", s, s.len());

//    let c = &s[0..1]; // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2027:5
    let c = &s[0..2];
    println!("First Unicode scalar of '{}' is '{}'", s, c);

    let sparkle_heart : Vec<u8> = vec![240, 159, 146, 150];
//    let sparkle_heart : Vec<u32> = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    println!("Sparkle heart = {}", sparkle_heart);

    let s = String::from("Здравствуйте");
    let strings = ["Здравствуйте", "नमस्ते"];
    for s in &strings {
        print!("{} in bytes: ", s);
        for b in s.bytes() {
            print!("{} ", b);
        }
        println!();
        print!("{} in chars: ", s);
        for c in s.chars() {
            print!("{} ", c);
        }
        println!();
    }
}
