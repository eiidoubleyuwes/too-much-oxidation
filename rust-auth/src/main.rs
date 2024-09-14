use reqwest::blocking::Client;
use reqwest::Error;
fn main() -> Result<(), Error> {
    let client = Client::new();
    let user = "Bigshlongmike".to_string();
    let passwrd: Option<String> = None;

    let response = client.get("http:httpbin.org/get").basic_auth(user, passwrd).send()?;
    //Lets filter for status codes
    let status = response.status();
    println!("Status: {:?}", status);
    println!("{:?}", response);
    Ok(())

} 
