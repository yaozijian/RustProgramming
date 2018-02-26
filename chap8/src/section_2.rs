
pub fn str_demo(){
	
	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2;
	let s = "abc";
	let s3 = s3 + s;
	println!("{}",s3);
	/*
	注意:
	(1) String 使用方法add 对 + 进行了重载
	
	(2) String::add()方法的签名为: fn add(self,&str) -> String
	所以执行串接操作后 s1 的所有权被转移，s1变得无效了；
	而对s2使用了引用，所以s2的所有权没有转移，s2仍然有效。
	
	(3) &s2的类型为&String，但在方法调用过程中使用解引用强制多态(deref coercion)技术
	将&String强转(coerced)成&str了，可理解将&s2变成了&s2[..]
	*/
	for c in s3.chars(){
		println!("{}",c);
	}
	
	for (i,c) in s3.bytes().enumerate(){
		println!("i={} c={}",i,c);
	}
}

