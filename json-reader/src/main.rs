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
                {
                "name": "Wahyu",
                "age": 20,
                "phones": [
                    "+62 812-3456-7890",
                    "+62 123-4567-8901"
                ]
            }
        ],
        "author": "Wahyu"
    }"#;
    let article: Article = serde_json::from_str(json).unwrap();
    println!("Author: {}", article.author);
    for paragraph in article.paragraphs {
        println!("Name: {}", paragraph.name);
        println!("Age: {}", paragraph.age);
        for phone in paragraph.phones {
            println!("Phone: {}", phone);
        }


    }
}
