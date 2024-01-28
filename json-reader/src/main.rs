//Today I'm making a project that alllows me to read json files and print them out in a nice format.
//I'm going to use serde_json to do this.
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Sentensi {
    name: String,
    age: u8,
    phones: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct Article{
    paragraphs: Vec<String>,
    author: String,
    author: String,
}
fn main() {
    println!("Hello,welsome to JSON Reader!");
    println!("Please enter the file name you want to read(Usage : File name): ");
    
}
