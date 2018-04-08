# 第12章 一个I/O项目：构建命令行程序

## 1 接收命令行参数

* 使用```std::env::args()```方法得到以迭代器表示的程序命令行参数
* 使用迭代器的```collect()```方法将迭代器转化成一个集合类型
* Rust中很少需要显式指明类型，但对于迭代器的```collect()```方法，通常需要指明结果类型，因为这个方法可以将迭代器中的数据转化成多种集合类型，Rust无法推断出你需要哪种类型的结果
* 示例：```let args : Vector<String> = std::env::args().collect();```
* 在模块中定义```main()```函数是合法的，如果用```pub```修饰，则这个函数可以被其他模块调用

## 2 读取文件

```rust
let mut file = File::open(filename).expect("file not found");
let mut content = String::new();
file.read_to_string(&mut content).expect("something went wrong when reading the file");
```

## 3 重构以改进模块化与错误处理

* 设置进程退出码

```rust
use std::process;
process::exit(1);
```

* ```Result```类型的一个辅助函数: ```unwrap_or_else```
  * 正常时返回封装的Ok值
  * 错误时调用闭包函数

```rust
let config = Config::new(&args).unwrap_or_else(|err|{...});
```

* 关于extern crate,use和mod
  * extern crate：用于导入非标准库中的包（crate），可以用```extern crate <包名> as <别名>```来为包定义别名
  * use：方便使用包中的类型和函数。使用extern crate导入包（标准包std不用导入）后，可以通过全名(包名::模块::类型/函数)使用包中的类型和函数，但不方便。使用use后可以方便地通过短名称来使用包中的类型和函数；同样地，可以带as字句来为类型和函数定义别名。
  * mod用于在包中定义模块
    1. 通常在单个文件中定义模块，编写模块代码；然后在其他文件中用mod <模块文件名(不带扩展名.rs)>引入模块
    2. 简单的模块可以直接写在源代码文件中: ```mod { 模块代码 }```
  * 通常将pub类型和函数，以及测试代码放在lib.rs中，而将内部使用的私有类型、函数、模块等放在其他文件中  

## 4 采用测试驱动开发完善库的功能

* 字面字符串支持多行，直接用双引号就可以，不用特别表示
* 字符串的```lines()```方法返回一个迭代器，将字符串切分成多行
* 字符串的```contains()```方法可判断是否包含子串

## 5 处理环境变量

* 字符串的```to_lowercase()```方法将每个组成字符变成小写字母
* ```std::env::var("变量名")```返回表示环境变量的Result类型值
* 初始化结构体变量时必须给出每个字段（不像Go那样自动将没有给出的字段初始化为空值)
* 灵活使用语句表达式(注意：不要多了分号)

```rust
let result = if config.case_insensitive{
    search(&config.query,&content)
}else{
    search_insensitive(&config.query,&content)
};
```

## 6 输出到标准错误输出

* 使用```eprintln!()```宏来将文本输出到标准错误输出
