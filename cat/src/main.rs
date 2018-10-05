use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(filename: String) -> Result<String, io::Error> {
    let mut f = try!(File::open(filename));
    let mut content = String::new();
    try!(f.read_to_string(&mut content));
    Ok(content)
}

fn print_args() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for i in &args[1..] {
            println!("{}", match read_file(i.clone()) {
                Ok(content) => content,
                Err(reason) => panic!(reason)
            });
        }
    }
}

fn main() {
    print_args();
}
