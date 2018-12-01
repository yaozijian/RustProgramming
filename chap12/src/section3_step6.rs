use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

struct Config {
    filename: String,
    query: String,
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("解析参数失败: {}",err);
        process::exit(1);
    });

    println!("文件: {} 查找: {}",config.filename,config.query);

    if let Err(err) = run(config){
        println!("读取文件失败: {}",err);
        process::exit(2);
    }
}

fn run(config : Config) -> Result<(),Box<Error>>{
	let mut content = String::new();
	File::open(config.filename)?.read_to_string(&mut content)?;
    println!("With text:\n{}", content);
    Ok(())
}

impl Config {
    fn new(args: &[String]) -> Result<Config,&'static str>{
        if args.len() < 3 {
            return Err("参数太少")
        }
        let filename = args[1].clone();
        let query = args[2].clone();
        Ok(Config {
            filename: filename,
            query: query,
        })
    }
}
