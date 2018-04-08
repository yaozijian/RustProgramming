
use std::env;

pub fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}",args);

    let filename = &args[1];
    let query = &args[2];

    println!("filename: {}",filename);
    println!("query: {}",query);
}