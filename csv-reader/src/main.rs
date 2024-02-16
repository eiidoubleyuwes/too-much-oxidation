//This tools should be able to read data from a csv file and print it out to the console
//ofc the csv file should be in the same directory as the executable
extern crate serde;
use std::fs;
use serde::Deserialize;
fn main() {
    std::process::exit( reader());
}
fn reader() -> i32{
    let args : Vec<_> = std::env::args().collect();
    if args.len() < 2{
        println!("Not enough arguments Usage: Filename");
        return 1;
    }
    #[derive(Debug, Deserialize)]
    struct Record{
       _name: String,
        _place: String,
        _id: String,
    }
    println!("Reading file {}",args[1]);
    let mut rdr = csv::Reader::from_reader(fs::File::open(&args[1]).unwrap());
    for result in rdr.deserialize(){
        let record: Record = result.unwrap();
        println!("{:?}",record);
    }
    let filename = &args[1];
    let file = fs::File::open(filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records(){
        let record = result.unwrap();
        println!("{:?}",record);
    }
    0
}
