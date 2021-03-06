use rust_training::helper;
use rust_training::hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// --------------------------------------------------------------------------------------------
// Recreate vec!
// --------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// --------------------------------------------------------------------------------------------
// HelloMacro
// --------------------------------------------------------------------------------------------

#[derive(HelloMacro)]
struct Pancakes;

// --------------------------------------------------------------------------------------------
// Main
// --------------------------------------------------------------------------------------------

pub fn main() {
    helper::subsection("macro_rules!");
    let v = vec2![1, 2, 3];

    println!("vec2 = {:?}", v);

    helper::subsection("HelloMacro");
    Pancakes::hello_macro();
}