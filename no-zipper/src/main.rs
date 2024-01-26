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
        let outpath = match file.enclosed_name(){
            Some(path) => path.to_owned(),
            None => continue,
        };
        let comment = file.comment();
        if !comment.is_empty(){
            println!("File {} comment: {}",i,comment);
        }
        if filename.is_empty(){
            println!("File {} extracted to \"{}\"",i,outpath.display());
        }
        else{
            println!("File {} extracted to \"{}\" (in file {})",i,outpath.display(),filename);
            fs::create_dir_all(outpath.parent().unwrap()).unwrap();
        }
        if let Some(p) = outpath.parent(){
            if !p.exists(){
                fs::create_dir_all(p).unwrap();
            }
        }
        let mut outfile = fs::File::create(&outpath).unwrap();
        io::copy(&mut file,&mut outfile).unwrap();
        println!("File {} extracted to \"{}\"",i,outpath.display());
    }
}
