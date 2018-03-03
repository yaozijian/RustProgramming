---
style: summer
---
# 第10章 泛型、特性与生命周期

## 1 泛型

### 1.0 不使用泛型

```rust
fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}
```

### 1.1 在函数签名中使用泛型

<font color="red">注意：这段代码通不过编译，因为泛型类型T需要满足```std::cmp::PartialOrd```才能使用大于号```>```，而函数签名没有限定这一点</font>

```rust
fn largest<T>(list: &[T]) -> T {
  let mut val = list[0];
  for &item in list.iter() {
    if item > val {
      val = item;
    }
  }
  val
}
```

### 1.2 在结构体中使用泛型

```rust
struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };
}
```

#### 1.2.1 使用多个泛型类型

```rust
struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let both_integer = Point { x: 5, y: 10 };
  let both_float = Point { x: 1.0, y: 4.0 };
  let integer_and_float = Point { x: 5, y: 4.0 };
}
```

#### 1.2.2 定义方法

```rust
impl<T> Point<T> {
  fn x(&self) -> &T {
      &self.x
  }
}
```

#### 1.2.3 为特定类型定义方法（特例化)

```rust
impl Point<f32> {
  fn distance_to_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```

#### 1.2.4 结构体方法带有不同的泛型参数

```rust
impl<T, U> Point2<T, U> {
  fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
    Point2 {
      x: self.x,
      y: other.y,
    }
  }
}
```

### 1.3 枚举定义中的泛型

```rust
enum Option<T> {
  Some(T),
  None,
}
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

### 1.4 泛型没有运行时开销

* 编译器在编译时对泛型代码进行单态化（monomorphization），即将泛型类型替换成具体类型。这会引起代码膨胀，但能够保证运行效率。
* 写程序时不用书写重复的代码，而运行效率与手写代码一样。

## 2 特性(trait)

### 2.1 定义与使用特性

```rust
pub trait Summarizable {
  fn summary(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summarizable for NewsArticle {
  fn summary(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summarizable for Tweet {
  fn summary(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub fn demo() {
  let obj1: &Summarizable = &Tweet {
    username: String::from("SunWuKong"),
    content: "DaNaoTianGong".to_string(),
    reply: false,
    retweet: false,
  };
  println!("{}", obj1.summary());
}
```

* 注意结构体与特性间的赋值：```let obj : &Summarizable = &Tweet{...}```，其他写法不正确
* 要为结构体实现某特性，被实现特性必须在相同作用域中，或者使用use将其导入
* 可以为其他包（crate）、模块中已经存在的类型实现特性
* 孤儿规则（orphan rule）：不能为外部包中的类型实现外部包中的特性，类型或者要实现的特性，二者至少有一个在本地作用域中。

### 2.2 特性方法的默认实现

#### 2.2.1 特性方法的默认实现

```rust
pub trait Summarizable {
  fn summary(&self) -> String {
      String::from("(Read more...)")
  }
}
impl Summarizable for NewsArticle {}
```

* 使用默认实现时只需要指定一个空的impl块

#### 2.2.2 默认实现可以调用其他没有默认实现的方法

```rust
pub trait Summarizable {
  fn author_summary(&self) -> String;
  fn summary(&self) -> String {
      format!("(Read more from {}...)", self.author_summary())
  }
}
impl Summarizable for Tweet {
  fn author_summary(&self) -> String {
      format!("@{}", self.username)
  }
}
```

<font color="red">

* 注意：为结构体实现特性的时候，无法调用默认实现。

</font>

### 2.3 特性限定(trait bounds)

#### 2.3.1 特性限定

* 可以指定实现泛型的具体类型必须实现了某些特性，这就是特性限定（trait bounds）

```rust
fn notify1<T:Summarizable>(item: &T){
  println!("独家新闻: {}",item.summary());
}

fn notify2<T>(item: &T) where T: Summarizable{
  println!("独家新闻: {}",item.summary());
}

fn notify3<T>(item: &T) where T: Summarizable + Debug{
  println!("独家新闻: {:?}",item.summary());
}
```

#### 2.3.2 使用特性限定有条件地为泛型结构体实现方法

```rust
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
        println!("The largest member is x = {}", self.x);
    } else {
        println!("The largest member is y = {}", self.y);
    }
  }
}
```

#### 2.3.3 总括实现(blanket implementation)

* 总括实现：为实现了某些特定特性的全体类型实现某些特性

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

## 3 生命周期

* 每一个变量都有 **生命周期（lifetime）**，但本节只讨论引用变量的生命周期
* 很多时候编译器可以自动进行生命周期推导，无需显式使用生命周期注解来声明生命周期

### 3.1 生命周期的作用

* 生命周期可避免悬垂引用，即释放后使用(use-after-free)

```rust
{
  let r;
  {
    let x = 5;
    r = &x;
  }
  println!("r: {}", r);
}
```

* 对上面的代码片段，编译器会检查出，最后的```println!```语句中的```r```引用了已经超出作用域的变量```x```，这就是悬垂引用
* 上面的代码片段中声明变量```r```时不进行初始化是合法的，但是如果随后在没有给```r```赋值前就使用它，则是不合法的
* 编译器中的 **借用检查器(borrow checker)** 对代码执行检查，确保代码符合生命周期规则

### 3.2 不使用生命周期注解

```rust
fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

pub fn demo() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}
```

* 上述代码中的```longest```函数通不过编译，因为编译器无法完成生命周期推导：不知道程序运行时实际参数```x```和```y```的值，无法得知执行```if```语句的哪个分支；也不知道参数的生命周期是怎样的，无法通过作用域来判断返回的引用是否有效

### 3.3 函数签名中的生命周期注解

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}
```

* 生命周期注解用于向编译器说明多个引用的生命周期间的关系，所以单个生命周期注解没有什么用，通常可以省略
* 生命周期注解只能用于引用，不能用于非引用变量
* 生命周期注解是一种泛型类型（可变参数类型是另一种泛型类型）
* 生命周期注解不能改变参数的生命周期，只是说明参数与返回值的生命周期之间必须满足的关系。如果调用函数时给出的参数不满足这种关系，可编译通不过。
* 生命周期注解只存在于函数签名中，而不存在函数体中。函数签名给出生命周期注解后，编译器就可以对函数体进行分析。函数每次被调用时，参数和返回值的生命周期都可能不同，编译器无法进行完全的分析，因为开销太大。

#### 3.3.1 借用检查器使用参数生命周期中最小的那一个进行检查

<font color="red">

* 借用检查器执行检查时，将生命周期```'a```替换成```x```和```y```的生命周期重叠的部分，即是```x```和```y```的生命周期中较小的那一个。

</font>

```rust
fn main() {
  let string1 = String::from("long string is long");
  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }
}
```

* ```string2```的生命周期较```string1```小，所以将```string2```的生命周期传入函数，进行生命周期检查。

```rust
fn main() {
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {}", result);
}
```

* 同理，实际传入的是```string2```的生命周期，所以返回值的生命周期等于```string2```的生命周期。在最后的```println!```中使用```result```是不正确的，因为```result```已经无效，代码无法编译通过。
* 注意：虽然调用函数```longest```时实际返回的是```string1```的引用，而```string1```的生命周期足够长，但是代码还是无法编译通过，因为编译器只能对代码进行静态的生命周期检查，而无法执行动态检查。只有程序动态运行起来之后，才知道实际返回的是```string1```的引用；编译器以函数为单位进行生命周期检查，而不检查函数间的调用关系。

```rust
fn main() {
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = "xyz";
    result = longest(string1.as_str(), string2);
  }
  println!("The longest string is {}", result);
}
```

* 上述代码是正确的，因为```string2```引用了字面字符串，而字面字符串的生命周期是```'static```，不短于其他任何变量的生命周期，所以借用检查器将```string1```的生命周期替换成```'a```,返回值```result```的生命周期等于```string1```的生命周期，最后的```println!```使用```result```是合法的。
* 注意：即使将```string2```赋值为一个更长的字符串，使得返回的是```string2```的值，代码也是可以编译通过，正常运行的。因为：
  * ```string2```只是一个```str```变量的引用，```string2```超出作用域时被释放并不会导致它所引用的字符串被释放，因为引用是没有所有权的。
  * 返回值是一个```str```变量的引用，其有效性与这个变量是否有其他引用（```string2```)，其他引用是否有效没有关系。

#### 3.3.2 与返回值无关的引用参数不需要生命周期注解

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str { x }
```

* 返回值与参数```y```没有关系，所以```y```不需要生命周期注解。

#### 3.3.3 引用返回值的生命周期必须与某个输入参数的生命周期相关

* 如果引用返回值的生命周期不与任何输入参数的生命周期相关，则返回值引用的是函数体中创建的值。函数调用返回后，函数体中创建的值变得无效，而返回值还在引用这个值，从而变成悬垂引用。
* 这种情况下不能返回引用，而必须返回拥有所有权的值类型。

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
  let result = String::from("really long string");
  result.as_str()
}
```

* 返回值与输入参数没有关系，从而函数签名中的生命周期注解是不需要的。
* 返回值引用的是函数体中创建的值，函数返回后，返回值变成悬垂引用，这个函数无法通过编译。
* 正确的写法如下(编译器会警告```x```和```y```未被使用，暂时忽略)：

```rust
fn longest(x: &str,y: &str)-> String{
  let result = String::from("really long string");
  result
}
```

<font color="red">
生命周期注解描述了函数输入参数与返回值的生命周期之间的关系。编译器根据生命周期注解对函数代码进行检查，允许内存安全的操作；阻止产生悬垂指针或是违反内存安全的操作。</font>

### 3.4 结构体中的生命周期注解

* 结构体含有引用类型的字段时，必须给出生命周期注解

```rust
struct ImportantExcerpt<'a> {
  part: &'a str,
}
```

### 3.5 生命周期省略（elision）

* 函数参数的生命周期称为“输入生命周期”；返回值的生命周期称为“输出生命周期”
* 生命周期省略规则
  1. 每个参数有各自不同的生命周期
  2. 如果只有一个输入生命周期，则将其作为输出生命周期
  3. 如果有多个输入生命周期，但是其中一个为```&self```或者```&mut self```,则将其作为输出生命周期
* 如果执行上述三条规则后，无法确定输出生命周期，则编译器停止编译，生成错误

#### 3.5.1 生命周期检查示例1

```rust
fn first_word(s: &str) -> &str {
```

* 根据第一条规则，函数签名变成：

```rust
fn first_word(s:&'a str) -> &str
```

* 根据第二条规则，函数签名变成：

```rust
fn first_word(s:&'a str) -> &'a str
```

* 至此，已经确定输出生命周期，函数签名合法，无需执行第三条规则（其实，第三条规则是无法应用的，因为没有```&self```或者```&mut self```参数）。

#### 3.5.2 生命周期检查示例2

```rust
fn longest(x: &str, y: &str) -> &str
```

* 根据第一条规则，函数签名变成：

```rust
fn longest(x:&'a str,y:&'b str) -> &str
```

* 因为有多个输入生命周期，第二条规则不适用
* 没有```&self```或者```&mut self```参数，第三条规则不适用。
* 至此，已经尝试使用全部三条规则，没有计算出输出生命周期，所以函数签名不合法，编译器停止编译，输出错误。

#### 3.5.3 方法中的生命周期注解

```rust
struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  // 没有返回引用类型，无需生命周期注解
  fn level(&self) -> i32 {
    3
  }
  // 可使用第三条规则计算出输出生命周期，可以省略生命周期注解
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}
```

### 3.6 静态生命周期

* 用```'static```表示静态生命周期，即生命周期等于整个程序运行周期
* 字面字符串的生命周期就是静态生命周期
* 通常不应该使用静态生命周期

## 4 泛型参数、特性限定和生命周期注解的混合写法

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```
