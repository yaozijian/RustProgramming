# 第13章 Rust中的函数式语言功能：迭代器与闭包

## 1 闭包：可以捕获环境的匿名函数

* 暂停执行：```std::thread::sleep(std::time::Duration::from_secs(2))```
* 原始程序如下，可能需要的改进有：
  * 只应该在必要的时候调用```simulated_expensive)calculation()```一次
  * intensity >= 25且random_number==3时应该避免调用```simulated_expensive)calculation()```

```rust

extern crate std;

pub fn main() {
    let simulated_user_specified_value = 10;
    let simutlated_random_number = 7;
    step0::generate_workout(simulated_user_specified_value, simutlated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32{
    println!("慢速计算中...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    intensity
}

mod step0{

    extern crate std;

    pub fn generate_workout(intensity: u32,random_number: u32){
        if intensity < 25{
            println!("做{}个俯卧撑",super::simulated_expensive_calculation(intensity));
            println!("做{}个仰卧起坐",super::simulated_expensive_calculation(intensity));
        }else if random_number == 3{
            println!("今天休息，注意补充水分");
        }else{
            println!("跑步{}分钟",super::simulated_expensive_calculation(intensity));
        }
    }
}
```

### 1.1 改进1：用变量存储调用```simulated_expensive_calculation()```的结果

* 好处：避免了连续两次调用simulated_expensive_calculation()
* 缺点：在不需要的时候(intensity >= 25 && random_number == 3)也调用了simulated_expensive_calculation()
* 需要闭包：在一个地方定义一些代码，在另一些需要的地方执行这些代码

```rust
mod step1{
    pub fn generate_workout(intensity: u32,random_number: u32){
        let result = super::simulated_expensive_calculation(intensity);
        if intensity < 25{
            println!("做{}个俯卧撑",result);
            println!("做{}个仰卧起坐",result);
        }else if random_number == 3{
            println!("今天休息，注意补充水分");
        }else{
            println!("跑步{}分钟",result);
        }
    }
}
```

### 1.2 改进2：用闭包代替simulated_expensive_calculation()函数

* 好处：避免了不需要的时候(intensity >= 25 && random_number == 3)调用simulated_expensive_calculation()
* 缺点：没有避免连续两次调用simulated_expensive_calculation()

```rust
mod step2{
    extern crate std;

    pub fn generate_workout(intensity: u32,random_number: u32){

        let result = |intensity|{
            println!("慢速计算中...");
            std::thread::sleep(std::time::Duration::from_secs(2));
            intensity
        };

        if intensity < 25{
            println!("做{}个俯卧撑",result(intensity));
            println!("做{}个仰卧起坐",result(intensity));
        }else if random_number == 3{
            println!("今天休息，注意补充水分");
        }else{
            println!("跑步{}分钟",result(intensity));
        }
    }
}
```

### 1.3 关于闭包

* 闭包定义中通常省略参数和返回值类型，而让编译器去进行自动类型推导
  * 闭包通常较小，用于特定上下文中，在这种特定上下文中，编译器通常可以进行自动类型推导；如果要求给出参数和返回值类型，写法会比较繁琐
  * 函数通常是给其他用户使用的，要求定义参数和返回值类型有助于所有用户对如何使用函数有一致的看法
* 当然，也可以为闭包的参数和返回值指定类型，只是写法比较繁琐

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

* 第一次调用闭包时，编译器对闭包进行类型推导；如果此后的调用与第一次推导出的类型不符，则发生编译错误。

```rust
let example_closure = |x| x;
let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

* 上面的例子中，第一次调用推导出参数x的类型是String；而第二次传入int类型的参数，发生类型不匹配，无法通过编译。

### 1.4 改进3：缓存计算结果以避免不必要的计算

* 定义一个存储闭包和缓存结果的结构体
* 每个闭包都有自己独特的匿名类型，无法在结构体中指定此匿名类型，只能用Fn特性族来指定闭包类型，就像第10章那样使用泛型和特性限定。
* 每个闭包和函数都实现了三个Fn特性中的某一个特性。
* Fn特性族
  * Fn：以不可变引用方式借用对象（结构体、闭包等)，self参数定义为：```&self```
  * FnMut：以可变引用方式借用对象（结构体、闭包等)，self参数定义为：```&mut self```
  * FnOnce：对象的所有权会转移到特性对象中，只能用对象调用一次这个特性，以后不能调用（因为没有对象的所有权了），self参数定义为：```self```

```rust
mod step3{
    extern crate std;

    struct Cache<T> where T:Fn(u32)->u32{
        calc: T,
        val: Option<u32>,
    }

    impl<T> Cache<T> where T:Fn(u32) -> u32{
        fn new(calc: T) -> Cache<T>{
            Cache{
                calc: calc,
                val: None,
            }
        }

        fn value(&mut self,arg: u32) -> u32{
            match self.val{
                None => {
                    let v = (self.calc)(arg);
                    self.val = Some(v);
                    v
                },
                Some(v) => v,
            }
        }
    }

    pub fn generate_workout(intensity: u32,random_number: u32){

        let calc = |intensity|{
            println!("慢速计算中...");
            std::thread::sleep(std::time::Duration::from_secs(2));
            intensity
        };
        let mut cache = Cache::new(calc);

        if intensity < 25{
            println!("做{}个俯卧撑",cache.value(intensity));
            println!("做{}个仰卧起坐",cache.value(intensity));
        }else if random_number == 3{
            println!("今天休息，注意补充水分");
        }else{
            println!("跑步{}分钟",cache.value(intensity));
        }
    }
}
```

### 1.5 练习：更多改进

#### 1.5.1 用HashMap存储多个不同参数对应的不同结果值

```rust
mod step4{
    extern crate std;

    use std::collections::HashMap;

    struct Cache<T> where T:Fn(u32)->u32{
        calc: T,
        store: HashMap<u32,u32>,
    }

    impl<T> Cache<T> where T:Fn(u32) -> u32{
        fn new(calc: T) -> Cache<T>{
            Cache{
                calc: calc,
                store: HashMap::new(),
            }
        }

        fn value(&mut self,arg: u32) -> u32{
            let v = self.store.entry(arg).or_insert((self.calc)(arg));
            *v
        }
    }

    pub fn generate_workout(intensity: u32,random_number: u32){

        let calc = |intensity|{
            println!("慢速计算中...");
            std::thread::sleep(std::time::Duration::from_secs(2));
            intensity
        };
        let mut cache = Cache::new(calc);

        if intensity < 25{
            println!("做{}个俯卧撑",cache.value(intensity));
            println!("做{}个仰卧起坐",cache.value(intensity));
        }else if random_number == 3{
            println!("今天休息，注意补充水分");
        }else{
            println!("跑步{}分钟",cache.value(intensity));
        }
    }
}
```

#### 1.5.2 在HashMap中使用泛型

```rust
mod step5{
    extern crate std;

    use std::collections::HashMap;

    struct Cache<T,K,V> where T:Fn(K)->V,K:Copy+Eq+std::hash::Hash,V:Copy{
        calc: T,
        store: HashMap<K,V>,
    }

    impl<T,K,V> Cache<T,K,V> where T:Fn(K) -> V,K:Copy+Eq+std::hash::Hash,V:Copy{
        fn new(calc: T) -> Cache<T,K,V>{
            Cache{
                calc: calc,
                store: HashMap::new(),
            }
        }

        fn value(&mut self,arg: K) -> V{
            let v = self.store.entry(arg).or_insert((self.calc)(arg));
            *v
        }
    }

    pub fn generate_workout(intensity: &str,random_number: u32){

        let calc = |info:&str| -> usize{
            println!("慢速计算中: {}...",info);
            std::thread::sleep(std::time::Duration::from_secs(2));
            10
        };
        let mut cache = Cache::new(calc);
        let inten = 10;

        if inten < 25{
            println!("做{}个俯卧撑",cache.value(intensity));
            println!("做{}个仰卧起坐",cache.value(intensity));
        }else if random_number == 3{
            println!("今天休息，注意补充水分");
        }else{
            println!("跑步{}分钟",cache.value(intensity));
        }
    }
}
```

### 1.6 闭包可以捕获环境

```rust
fn main() {
    let x = 4;
    // 闭包可以捕获环境：可以在闭包中使用外部定义的变量x
    let equal_to_x = |z| z == x;
    // 函数不可以捕获环境：以下函数使用了未定义的变量x（或者说试图像闭包那样使用外部定义的变量x），无法通过编译
    fn equal_to_x2(z: i32) -> bool { z == x}
    let y = 4;
    assert!(equal_to_x(y));
}
```

* 闭包通过三种方式之一捕获环境，这对应三种给函数传递参数的方式
  * Fn：以不可变引用方式借用对象（结构体、闭包等)，self参数定义为：```&self```
  * FnMut：以可变引用方式借用对象（结构体、闭包等)，self参数定义为：```&mut self```
  * FnOnce：对象的所有权会转移到特性对象中，只能用对象调用一次这个特性，以后不能调用（因为没有对象的所有权了），self参数定义为：```self```
* 默认情况下，编译器根据闭包代码对变量的使用方式来确定采用哪种方式进行捕获。如果使用move关键字，则可以明确告诉编译器采用FnOnce方式捕获闭包使用的变量，即将变量的所有权转移到闭包中。

```rust
fn main() {
    let x = vec![1, 2, 3];
    // 使用move关键字要求将变量所有权转移到闭包中
    let equal_to_x = move |z| z == x;
    // x的所有权已经转移到闭包中,x已经无效，下面这句访问x是错误的，代码通不过编译
    println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
```

## 2 使用迭代器处理元素序列

### 2.1 从集合类型创建迭代器

通常集合类型提供了三个获取迭代器的方法

* ```iter()```：生成不可变引用的迭代器
* ```iter_mut()```：生成可变引用的迭代器
* ```into_iter()```：获取所有权，返回拥有所有权的迭代器

通常在for循环中使用迭代器：

```rust
fn step1(){
    let v = vec![1,2,3];
    {
        let i = v.iter();
        for x in i{
            println!("{}",x);
        }
    }
    // 注意: 这个for循环会获取v的所有权,结果:
    // 1 上面的 for 循环必须用大括号包围,否则上面的v.iter()对v进行不可变借用,而这里要获取v的所有权，这是不允许的
    // 2 下面的 println! 语句无法使用 v，因为v的所有权已经被for语句获取，for之后v已经无效
    for x in v{
        println!("{}",x);
    }

    //println!("{:?}",v);
}
```

<font color="red">

* 注意：直接对集合类型示例v使用for循环时，相当于```for i in v.into_iter()```
* 而```v.into_iter()```会获取v的所有权，即v的所有权被转移到for语句中
* 结果是：for语句之后无法再使用v，因为所有权已经被转移，v已经无效

</font>

### 2.2 Iterator

* 所有迭代器都实现了std::iter::Iterator这个特性

```rust
trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

* 上述Item和Self::Item定义了Iterator特性的关联类型(associated type)
* 通常通过for循环简介调用next()方法，但是也可以直接调用

```rust
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

#### 2.2.1 消费适配器

* 除了next方法，Iterator还定义了其他一些调用next实现其功能的方法，这些方法称作消费适配器
* 注意：消费适配器通常会获取迭代器的所有权，调用消费适配器方法后，迭代器变量就无效了
* 常用的消费适配器
  * ```fn count(self) -> usize```
  * ```fn last(self) -> Option<Self::Item>```
  * ```fn nth(self,n: usize) -> Option<Self::Item>```
  * ```fn for_each<F>(self,f:F) where F : FnMut(Self::Item)```
  * ```fn all<F>(self,f:F) -> bool where F : Fn(Self::Item) -> bool```
  * ```fn any<F>(self,f:F) -> bool where F : Fn(Self::Item) -> bool```
  * ```fn find<F>(&mut self,predicate:F) -> Option<Self::Item> where F : FnMut(&Self::Item) -> bool```
  * ```fn position<P>(&mut self,predicate:P) -> Option<usize> where P : FnMut(Self::Item) -> bool```
  * ```fn rposition<P>(&mut self,predicate:P) -> Option<usize> where P : FnMut(Self::Item) -> bool```
  * ```fn max(self) -> Option<Self::Item> where Self::Item : Ord```
  * ```fn min(self) -> Option<Self::Item> where Self::Item : Ord```
  * ```fn sum<S>(self) -> S where S: Sum<Self::Item>```
* 示例

```rust
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
```

#### 2.2.2 迭代适配器

* Iterator特性定义了一些方法，可以返回其他不同类型的迭代器，这些方法称作迭代适配器
* 注意：迭代适配器通常会获取迭代器的所有权，调用迭代适配器方法后，迭代器变量就无效了
* 注意：迭代器是惰性的，如果不为返回的迭代器调用一个消费适配器方法，则编译器会给出警告
* 常用的迭代适配器
  * ```fn map<B,F>(self,f:F) -> Map<Self,F> where F:FnMut(Self::Item) -> B```
  * ```fn step_by(self,step : usize) -> StepBy<Self>```
  * ```fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> where U: IntoIterator<Item = Self::Item>```
  * ```fn zip(self)```
  * ```fn filter(self)```
  * ```fn filter_map(self)```
  * ```fn peekable(self)```
  * ```fn skip_while(self)```
  * ```fn take_while(self)```
  * ```fn skip(self,n:usize)```
  * ```fn task(self,n:usize)```
* 示例

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

### 2.3 通过实现Iterator特性来创建自定义迭代器

```rust
struct Counter{
    count : u32,
}

impl Counter {
    fn new() -> Counter{
        Counter{count: 0}
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<u32>{
        if self.count < 5{
            self.count += 1;
            Some(self.count)
        }else{
            None
        }
    }
}

let sum : u32 = Counter::new().zip(Counter::new().skip(1))
                .map(|(a,b)|a*b)
                .filter(|x| x % 3 == 0)
                .sum();
assert_eq!(sum,18);
```









