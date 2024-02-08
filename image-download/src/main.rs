use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;
//To handle all the errors we eill make from file creation to HTTP reqwest
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
#[tokio::main] // for the async func
async fn main() -> Result<()> {
    let tm_dir = Builder::new().prefix("Baraka'sdownloader").tempdir()?; // create a temp directory
    let response = reqwest::get("").await?;
   
    
}
