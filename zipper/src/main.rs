extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::io::copy;
use std::env::{args};
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    //Expect three items Usage,source and target
   if args().len() != 3 {
      println!("Usage: `source` `target`");
      return;
   }
   let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
   let output = File::create(args().nth(2).unwrap()).unwrap();

}
