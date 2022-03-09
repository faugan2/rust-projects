use std::io;
use std::fs::File;
use std::io::prelude::*;

fn read_file()->Result<String,io::Error>{
    let mut s=String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
pub fn run(){
    let file=read_file();
    println!("file={:?}",file);
}