//This simple program is for a simple password generator in Rust
//It will generate a random password of a given length
//The password will contain a mix of uppercase, lowercase, numbers and special characters
//The password will be printed to the console
//The password will be saved to a file called "password.txt"

use rand::Rng;
use std::io::Write;

fn main() {
    //Allow user input
    println!("Enter the desired password length: ");
    let mut password_length = String::new();
    std::io::stdin().read_line(&mut password_length).unwrap();
    let password_length: u32 = password_length.trim().parse().unwrap();
    // let password_length = 20; // Set the desired password length
    let mut password = String::new();

    for _ in 0..password_length {
        let random_char = rand::thread_rng().gen_range(33..127);
        password.push(char::from_u32(random_char as u32).unwrap());
    }

    println!("Generated password: {}", password);

    let mut file = std::fs::File::create("password.txt").unwrap();
    file.write_all(password.as_bytes()).unwrap();
}