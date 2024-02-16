//This project is a simple rust project for me to learn how to write JSON files in rust.
//I will be using the serde_json crate to do this.
//I will also be using the serde crate to serialize the data.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
    
}
#[derive(Serialize, Deserialize)]
struct Article {
   article: String,
   author: String,
   paragraph: Vec<Paragraph>,
}

fn main() {
    let article = Article {
        article: "This is a test article".to_string(),
        author: "Baraka Mbugua".to_string(),
        paragraph: vec![
            Paragraph {
                name: "I'm learning Rust".to_string(),

            },
            Paragraph {
                name: "This is very difficult".to_string(),
            },
            Paragraph {
                name: "I might just cry".to_string(),
            },
        ],
    };
    //Lets see if I can print JSON to the screen
    let json = serde_json::to_string(&article).unwrap();
    println!("{}
    ", json);
    //Now lets see if I can read the JSON file
    let article: Article = serde_json::from_str(&json).unwrap();
    println!("{}
    ", article.article);
    println!("{}
    ", article.author);
    println!("{}
    ", article.paragraph[0].name);
    println!("{}
    ", article.paragraph[1].name);
    println!("{}
    ", article.paragraph[2].name);



}
