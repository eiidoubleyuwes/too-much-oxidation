//This project is a simple rust project for me to learn how to write JSON files in rust.
//I will be using the serde_json crate to do this.
//I will also be using the serde crate to serialize the data.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
    
}
struct Article {
   article = String,
   author = String,
   paragraph = vec<Paragraph>,
}

fn main() {
    let article = Article {
        article: "This is a test article".to_string(),
        author: "John Doe".to_string(),
        paragraph: vec![
            Paragraph {
                name: "This is a paragraph".to_string(),

            },
            Paragraph {
                name: "This is another paragraph".to_string(),
            },
            Paragraph {
                name: "This is a third paragraph".to_string(),
            },
        ],
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("{}", json);


}
