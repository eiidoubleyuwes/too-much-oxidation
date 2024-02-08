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