pub fn add_some(x: Option<u32>, y: Option<u32>) -> Option<u32> {
    match (x,y) {
        (Some(a), Some(b)) => Some(a+b),
        (_,_)               => None
    }
}
