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
    paragraphs: Vec<Sentensi>,
    author: String,
    author: String,
}
fn main() {
    let json =r#"{
        "paragraphs": [
            {
                "name": "Wahyu",
                "age": 20,
                "phones": [
                    "+62 812-3456-7890",
                    "+62 123-4567-8901"
                ]
            },
        

    }
