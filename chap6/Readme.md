
# 1 枚举

* 枚举可以直接关联其他类型、元组、结构体
```
enum Message{
	Quit,
	Move{x: i32,y: i32},
	Write(String),
	Color(i32,i32,i32),
}
```
* 注意涉及到String时match子句的写法，必要时候使用ref
```
fn show2(&self){
            match self{
                &Message::Quit => println!("Quit"),
                &Message::Move{x:x1,y:y1} => println!("Move to ({},{})",x1,y1),
                // 注意: 这里必须写成ref msg,表示msg变量采用引用
                // 注意：不能在这里指定变量类型，不能写msg: &String
                &Message::Write(ref msg) => println!("Write msg: {}",msg),
                &Message::Color(r,g,b) => println!("Set Color to ({},{},{})",r,g,b),
            }
        }
```
* 不能为match子句中的变量指定类型，必要时候可以使用ref限定变量为引用

# 2 Option

* 标准库中的std::option模块定义了一个带泛型的枚举类型Option
```
enum Option<T> {
    Some(T),
    None,
}
```

* Option类型的一些方法

  * ```fn is_some(&self) -> bool```
  * ```fn is_none(&self) -> bool```
  * ```fn map<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> U```
  * ```fn map_or<U, F>(self, default: U, f: F) -> U where F: FnOnce(T) -> U```
  * ```fn map_or_else<U, D, F>(self, default: D, f: F) -> U where D: FnOnce() -> U,F: FnOnce(T) -> U ```
  * ```fn ok_or<E>(self, err: E) -> Result<T, E>```
  * ```fn ok_or_else<E, F>(self, err: F) -> Result<T, E> where F: FnOnce() -> E```
  * ```fn and<U>(self, optb: Option<U>) -> Option<U>```
  * ```fn and_then<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> Option<U>```
  * ```fn filter<P>(self, predicate: P) -> Option<T> where P: FnOnce(&T) -> bool```
  * ```fn take(&mut self) -> Option<T>```

# 3 match

* match的子句必须穷尽(exhaustive)所有情况，有时候可以用下划线_表示所有其他情况，以满足穷尽所有情况的要求
* 可以用if let来处理只关心一种匹配情况的场合
* if let可以带一个else子句，相当于match中用下划线匹配的所有其他情况




