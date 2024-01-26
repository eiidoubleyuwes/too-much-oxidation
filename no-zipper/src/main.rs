//this program is for unzipping a file
//It is meant to be complimentary to the one I made to zip files
use std::fs;
use std::io;
fn main(){
    std::process::exit(unzip())
}
fn unzip() -> i32{
    //Using a vector for args
    let args : Vec<_> = std::env::args().collect();
    //If there are not enough args
    if args.len() < 2{
        println!("Not enough arguments Usage: Filename",args[0]);
        return 1;
    }
    let fname =std::path::Path::new(&*args[1]);
}
