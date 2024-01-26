//this program is for unzipping a file
//It is meant to be complimentary to the one I made to zip files
use std::fs;
use std::io;
use std::iter::zip;
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
    let file = fs::File::open(fname).unwrap();
    //use zip archive
    let mut archive = zip::ZipArchive::new(file).unwrap();
    //for each file in the archive
    for i in 0..archive.len(){
        //get the file
        let mut file = archive.by_index(i).unwrap();
        //get the name of the file
        let fname = file.name();
        //create the file
        let mut outfile = fs::File::create(fname).unwrap();
        //copy the file
        io::copy(&mut file, &mut outfile).unwrap();
    }
}
