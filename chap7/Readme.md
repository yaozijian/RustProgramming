# 1 模块文件系统规则

* 如果模块foo没有子模块，则应该将模块foo的代码放在foo.rs文件中
* 如果模块foo有子模块，则应该讲模块foo的代码放在foo/mod.rs文件中
* lib.rs 用于定义整个包的顶层模块，不能与mod.rs混淆

# 2 引用外部包

* 代码中增加 ```extern crate 包名```
* cargo.toml 中增加外部包定义
* 关于本地包
  * ```comm = { path = "../communicator" }```
  * 包名由目标(被调用)包的cargo.toml定义，不可以被重命名
  * 包含目标包的目录名不一定等于目标包名

# 3 use

* use 可跟随一系列::分割的包名、模块名、符号名
* ```use TrafficLight::{Red, Yellow};```
* 引用模块下的所有符号：```use TrafficLight::*;```
* 引用父模块：```super::client```
* 从根模块开始引用：```::client```