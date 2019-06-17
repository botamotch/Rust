use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn write_to_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, data: Vec<C>) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    for l in data {
        file.write_all(l.as_ref())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

pub fn print_hello() {
    println!("> test mod print");
}

