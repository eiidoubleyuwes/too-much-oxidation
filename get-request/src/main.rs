use error_chain::error_chain;
use std::io::Read;

error_chain 

fn main() {
    let mut request = reqwest::get("https://httpbin.org/ip")?
        .expect("Couldn't make request");
    
}