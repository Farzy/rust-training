use std::fmt::Display;

fn show_all(v: Vec<&dyn Display>) {
    for item in v {
        println!("{}", item);
    }
}

pub fn dynamic_display() {
    let v = vec![&12 as &dyn Display, &"Hi!" as &dyn Display];

    show_all(v);
}
