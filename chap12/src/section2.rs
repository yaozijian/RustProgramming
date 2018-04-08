
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}",args);

    let filename = &args[1];
    let query = &args[2];

    println!("filename: {}",filename);
    println!("query: {}",query);

    let mut file = File::open(filename).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("something went wrong when reading the file");

    println!("With text:\n{}",content);
}