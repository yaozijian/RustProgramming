use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    filename: String,
    query: String,
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = parse_config(&args);

    println!("文件: {} 查找: {}",config.filename,config.query);

    let mut file = File::open(config.filename).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("something went wrong when reading the file");

    println!("With text:\n{}", content);
}

fn parse_config(args: &[String]) -> Config {
    let filename = args[1].clone();
    let query = args[2].clone();
    Config {
        filename: filename,
        query: query,
    }
}
