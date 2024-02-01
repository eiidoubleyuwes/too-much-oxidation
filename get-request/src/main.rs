use error_chain::error_chain;
use std::io::Read;

error_chain 

fn main() {
    let mut file = std::fs::File::open("foo.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}