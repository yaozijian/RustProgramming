use std::io;
use std::io::Read;
use std::fs::File;

#[allow(unused_must_use)]
fn main(){
	demo1();
	demo2();
	read_username_from_file();
	read_username_from_file_simple();
	read_username_from_file_simple2();
}

fn demo1(){
	match File::open("hello.txt"){
		Ok(file) => file,
		Err(ref error) if error.kind() == std::io::ErrorKind::NotFound =>{
			match File::create("hello.txt"){
				Ok(file)=>file,
				Err(error)=>panic!("创建文件失败: {:?}",error),
			}
		},
		Err(error) => panic!("打开文件失败: {:?}",error),
	};
}

// Result类型的辅助方法
fn demo2(){
	File::open("hello.txt").unwrap();
	File::open("hello.txt").expect("打开文件失败");
}

// 传播错误而不是调用panic!
fn read_username_from_file() -> Result<String, io::Error> {

	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

// 传播错误的简写
fn read_username_from_file_simple() -> Result<String,io::Error>{
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

#[allow(unused_must_use)]
fn read_username_from_file_simple2() -> Result<String,io::Error>{
	let mut s = String::new();
	File::open("hello.txt")?.read_to_string(&mut s);
	Ok(s)
}
