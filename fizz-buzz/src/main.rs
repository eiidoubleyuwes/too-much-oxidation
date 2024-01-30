//This is a simple take on the FizzBuzz problem
use std::io;
fn main() {
    println!("Welcome to fizz buzz,enter a number,to make it simple keep it between 1 and 100");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Bad input");
}
