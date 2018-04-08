---
style: summer
---
# 第11章 测试

## 1 入门

* 在函数前加上```#[test]```就可以使之成为测试函数
* 使用```cargo test```执行测试，测试过程会运行每个测试函数
* 测试中常用的宏
  * ```panic!```
  * ```assert!```，多余的参数会传递给```format!```，在断言失败时输出
  * ```assert_eq!```和```assert_ne!```，多余的参数会传递给```format!```，在断言失败时输出
  * ```#[should_panic]```指示测试函数应该panic：在发生panic时测试通过；不发生则不通过
  * ```#[should_panic(expected="预期的panic消息")]```设置预期的panic消息

## 2 控制测试执行

* ```cargo test <cargo test参数> -- <测试程序参数>```
* 默认并行运行各个测试函数，用```cargo test -- --test-threads=1```串行执行
* 默认情况下，测试通过时，仅显示测试函数名称，不显示函数运行过程中的标准输出；测试没有通过时，会显示测试函数执行的标准输出。
* 使用```--nocapture````表示对于测试通过的函数，也显示其标准输出
* 默认情况下会运行每个测试函数，使用cargo test <测试函数名>来执行单个测试，或者执行名称中含有指定字符串的所有测试函数
* 用```#[ignore]```来指示通常状态下忽略这个测试函数；用```cargo test -- --ignored```来仅仅执行通常状态下忽略的测试函数

## 3 测试的组织结构

* 通常在每个文件中增加用```#[cfg(tests)]```修饰的```tests```模块，在其中放置单元测试函数

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

* 使用```use super::*```导入外层模块中的所有函数后，就可以直接调用外层模块中的所有函数（包括私有函数）
* 在与src目录同级的tests目录中存放集成测试代码，tests目录中的每个文件都会当做单独的crate进行编译。
* 集成测试文件需要使用```extern crate 包名称```来导入被测试的包；而单元测试则不需要，因为单元测试代码就在被测试包中
* 使用```cargo test --test <集成测试文件名>```来仅仅执行某个集成测试
* 不能为可执行程序项目创建集成测试；只能对crate项目创建集成测试

### 集成测试示例

```rust
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

* tests/common/mod.rs 内容如下

```rust
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```
