
extern crate std;

pub fn main() {
    let simulated_user_specified_value = 10;
    let simutlated_random_number = 7;
    let step = 5;
    match step{
        0 => step0::generate_workout(simulated_user_specified_value, simutlated_random_number),
        1 => step1::generate_workout(simulated_user_specified_value, simutlated_random_number),
        2 => step2::generate_workout(simulated_user_specified_value, simutlated_random_number),
        3 => step3::generate_workout(simulated_user_specified_value, simutlated_random_number),
        4 => step4::generate_workout(simulated_user_specified_value, simutlated_random_number),
        5 => step5::generate_workout("step5", simutlated_random_number),
        _ => (),
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32{
    println!("慢速计算中...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    intensity
}

mod step0{
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

// 改进1：用变量缓存调用simulated_expensive_calculation()的结果
// 好处：避免了连续两次调用simulated_expensive_calculation()
// 缺点：在不需要的时候(intensity >= 25 && random_number == 3)也调用了simulated_expensive_calculation()
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

// 改进2：用闭包代替simulated_expensive_calculation()函数
// 好处：避免了不需要的时候(intensity >= 25 && random_number == 3)调用simulated_expensive_calculation()
// 缺点：没有避免连续两次调用simulated_expensive_calculation()
mod step2{
    extern crate std;

    pub fn generate_workout(intensity: u32,random_number: u32){
        
        let calc = |intensity|{
            println!("慢速计算中...");
            std::thread::sleep(std::time::Duration::from_secs(2));
            intensity
        };
        
        if intensity < 25{
            println!("做{}个俯卧撑",calc(intensity));
            println!("做{}个仰卧起坐",calc(intensity));
        }else if random_number == 3{
            println!("今天休息，注意补充水分");
        }else{
            println!("跑步{}分钟",calc(intensity));
        }
    }
}

// 改进3：缓存计算结果以避免不必要的计算
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