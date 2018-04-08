
mod section1;
mod section2;
mod section3_step1;
mod section3_step2;
mod section3_step3;
mod section3_step4;
mod section3_step5;
mod section3_step6;

use std::env as vne;
use std::process;
extern crate chap12;

fn main() 
{
    section1::main();
    section2::main();
    section3_step1::main();
    section3_step2::main();
    section3_step3::main();
    section3_step4::main();
    section3_step5::main();
    section3_step6::main();
    section3_step7();
}

fn section3_step7() {
    
    let args: Vec<String> = vne::args().collect();
    println!("{:?}", args);

    let config = chap12::Config::new(&args).unwrap_or_else(|err|{
        eprintln!("解析参数失败: {}",err);
        process::exit(1);
    });

    println!("文件: {} 查找: {}",config.filename,config.query);

    if let Err(err) = chap12::run(config){
        eprintln!("读取文件失败: {}",err);
        process::exit(2);
    }
}

