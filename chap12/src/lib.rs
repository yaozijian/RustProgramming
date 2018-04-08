
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[cfg(test)]
mod tests;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_insensitive: bool,
}

pub fn run(config : Config) -> Result<(),Box<Error>>{
    
    let mut file = File::open(config.filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("With text:\n{}", content);
    
    let result = if config.case_insensitive{
        search(&config.query,&content)
    }else{
        search_insensitive(&config.query,&content)
    };

    println!("结果: {:?}",result);
    
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&'static str>{
        if args.len() < 3 {
            return Err("参数太少")
        }
        let filename = args[1].clone();
        let query = args[2].clone();
        Ok(Config {
            filename: filename,
            query: query,
            case_insensitive: std::env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn search<'a>(query:&str,content:&'a str) -> Vec<&'a str>{

    let mut result = Vec::new();

    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }

    result
}

pub fn search_insensitive<'a>(query:&str,content:&'a str) -> Vec<&'a str>{

    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }

    result
}
