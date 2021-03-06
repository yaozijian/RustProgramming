# 第11章 测试

## 1 测试入门

### 1.1 增加测试代码

```rust
#[cfg(test)]
mod tests {
	#[test]
	fn exploration() {
		assert_eq!(2 + 2, 4);
	}
}
```

### 1.2 运行测试

* 用`cargo test`命令执行测试
* 每个测试函数在单独的线程中运行，一个测试不通过不会影响其他测试

```
E:\RustProjects\playground>cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running target\debug\deps\playground-73f3d496fde692d7.exe

running 1 test
// 测试模块名::测试函数名
test tests::exploration ... ok

// measured 用于性能测试
// filtered out 指示被过滤而不运行的测试数量
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 1.3 用于测试的宏

* `assert!`、`assert_eq!`、`assert_ne!`的最后一个可选参数用于指定用户定义的错误描述
* 带前缀`debug_`的`assert`系列宏表示：仅在调试版本有效，或者在指定了编译参数`-C debug-assertions`时有效
* `unreachable!`
* 将`#[should_panic]`加在测试函数前面，表示要求测试函数`panic!`
* `#[should_panic(expected = "Guess value must be less than or equal to 100")]`可以带第二个参数，表示预期的错误文本，避免放过其他未预期的错误

## 2 控制测试运行

* `cargo test --help`显示帮助
* `cargo test <给cargo test的参数> -- <给编译生成的测试二进制程序的参数>`
* 设置并行度为1，让多个测试函数串行执行：`cargo test -- --test-threads=1`
* 默认情况下，如果测试通过，则不显示测试函数的输出；`cargo test -- --nocapture`表示对于通过测试的函数，也输出函数的打印输出
* `cargo test <测试函数名称>`用于仅运行指定名称的测试函数，其中`<测试函数名称>`可以是函数名称的一部分（子串）
* 通常情况下不会运行用`#[ignore]`标记的测试函数，而`cargo test -- --ignored`表示仅运行被标记为`#[ignore]`的测试函数


```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

## 3 测试代码的组织

### 3.1 单元测试

* 标记为`#[test]`的函数不会被编译到二进制程序中，而仅用于测试
* 标记为`#[cfg(test)]`的模块不会被编译到二进制程序中，而仅用于测试
* 使用测试模块的好处是：可以增加一些测试辅助函数，而这些函数不会被编译到二进制程序中

### 3.2 测试私有函数

* 根据第7章描述的访问控制规则
   1. 测试函数可以访问所在模块的其他私有函数
   2. 测试模块可以访问父模块的私有函数
* 所以：测试函数可以测试私有函数。测试模块可以用`use super::*`来简化对父模块私有函数的访问。

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

### 3.3 集成测试

* 在项目根目录创建一个`tests`目录，与`src`目录同级，在`tests`目录中存放所有集成测试文件
* 必须用`extern crate <包名>`导入被测试的包


#### 3.3.1 可执行项目的测试

* 对于二进制项目，如果只有`src/main.rs`，则集成测试代码没法将程序代码作为`crate`导入（只有），没法进行测试
* 所以，建议对于二进制项目，也将主要代码放在`src/lib.rs`中，以方便编写集成测试代码
