---
style: summer
---
# 1 panic

* 默认情况下使用panic!宏将导致展开(unwinding)操作，程序会逐级回溯，执行清理工作。如果要让可执行程序较小，可禁止展开，直接崩溃(abort），方法是在cargo.toml中增加以下配置：
```
[profile.release]
panic='abort'
```
* 默认情况下panic时不会输出调用栈，如果设置环境变量`RUST_BACKTRACE=1`则会输出调用栈

# 2 Result

* ```std::result``` 模块定义了Result枚举，通常用于表示可能失败的返回值

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
* 打开文件示例

```
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
```
* Result类型的辅助函数
  * unwrap： 对于Ok值，直接返回；对于Err值，调用panic
  * expect：对于Ok值，直接返回；对于Err值，使用给定的字符串值调用panic
```
File::open("hello.txt").unwrap();
File::open("hello.txt").expect("打开文件失败");
```
* 不调用panic!宏，而是将错误返回给调用者（传播错误）

```
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
```
* 传播错误的简写

```
fn read_username_from_file_simple() -> Result<String,io::Error>{
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}
```
<font color="red">

* ```?```会尝试使用```From```特征来将错误类型转换成函数返回的错误类型

</font><font color="red">
* ```?```只能用在返回```Result```类型的函数中，在其他函数中使用会通不过编译
</font>

* ```?```支持链式方法调用

```
fn read_username_from_file_simple2() -> Result<String,io::Error>{
	let mut s = String::new();
	File::open("hello.txt")?.read_to_string(&mut s);
	Ok(s)
}
```