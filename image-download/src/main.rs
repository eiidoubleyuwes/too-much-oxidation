use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;
use std::io;
//To handle all the errors we eill make from file creation to HTTP reqwest
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
#[tokio::main] // for the async func
async fn main() -> Result<()> {
    println!("Enter the URL of the image you want to download: ");
    let mut target = String::new();
    io::stdin().read_line(&mut target)?;
    let target = target.trim(); // remove the trailing newline
    let tm_dir = Builder::new().prefix("Baraka'sdownloader").tempdir()?; // create a temp directory
    let response = reqwest::get(target).await?;

    //Dealing with the response
    let mut dest{
        let majina = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");
    println!("The file to print: '{}'", majina);
    let majina = tm_dir.path().join(majina);
    println!("File will be located under '{}'", majina.display());
      File::create(majina)?;
    };
    
   
}
