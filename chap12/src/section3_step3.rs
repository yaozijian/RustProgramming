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

    let config = Config::new(&args);

    println!("文件: {} 查找: {}",config.filename,config.query);

    let mut file = File::open(config.filename).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("something went wrong when reading the file");

    println!("With text:\n{}", content);
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("参数太少")
        }
        let filename = args[1].clone();
        let query = args[2].clone();
        Config {
            filename: filename,
            query: query,
        }
    }
}
