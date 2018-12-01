
mod section1;
mod section2;

fn main(){
    section1::main();
    section2::main();
	println!("{}",demo(|a,x| a+x));
}

fn demo<F>(f: F) -> i32 where F : Fn(i32,i32)->i32{
	vec![1,2,3].iter().fold(0,f)
}