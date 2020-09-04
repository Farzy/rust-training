use std::io;
use std::io::Read;
use std::fs::File;
use rust_training::helper;

pub fn main() -> io::Result<()> {
    helper::subsection("Read file /etc/services");

    let mut f = File::open("/etc/services")?;
    let mut buffer = Vec::new();
    let mut other_buffer = Vec::new();

    {
        let reference = f.by_ref();

        // read at most 5 bytes
        reference.take(50).read_to_end(&mut buffer)?;

    } // drop our &mut reference so we can use f again

    // original file still usable, read the rest
    f.read_to_end(&mut other_buffer)?;

    println!("First 50 bytes of file: {:?}", String::from_utf8(buffer).unwrap());
    println!("Length of rest of file: {}", other_buffer.len());
    Ok(())
}
