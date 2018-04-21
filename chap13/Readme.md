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