use error_chain::error_chain;
use std::io::Read;

fn main() {
    let mut request = reqwest::get("https://httpbin.org/ip")?
        .expect("Couldn't make request");
    let mut body = String::new();
    request.read_to_string(&mut body)?;
    println!("{}", body);
    Ok(())
}