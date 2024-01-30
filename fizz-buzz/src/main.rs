//This is a simple take on the FizzBuzz problem
use std::io;
fn main() {
    println!("Welcome to fizz buzz,enter a number,to make it simple keep it between 1 and 100");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Bad input");
    let input: i32 = input.trim().parse().expect("Bad input");
    for i in 1..input {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
