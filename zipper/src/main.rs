extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::copy;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    //Expect three items Usage,source and target
   if args().len() != 3 {
      println!("This is how you use it: `source(File to be compressed)` `target(File that was compressed)`");
      return;
   }
   let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
   let output = File::create(args().nth(2).unwrap()).unwrap();
   let mut encoder = GzEncoder::new(output, Compression::default());
   let start = Instant::now();
   copy(&mut input, &mut encoder).unwrap();
   encoder.finish().unwrap();
   let duration = start.elapsed();
   println!("{:?}",duration);
   println!("Thank you for using Baraka's compression tool,have a great day!!");
}
