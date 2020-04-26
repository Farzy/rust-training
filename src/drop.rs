struct Noisy {
    id: i32,
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisy number {} going out of scope!", self.id);
    }
}

pub fn droppy() {
    let _n1 = Noisy { id: 1};
    let _n2 = Noisy { id: 2};
    println!("End of droppy function.");
}
