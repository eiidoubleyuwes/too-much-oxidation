extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::io;
use std::env::Args;
