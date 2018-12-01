pub fn main(){
    step1();
    step2();
}

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

fn step2(){

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
        fn next(&mut self) -> Option<Self::Item>{
            if self.count < 5{
                self.count += 1;
                Some(self.count)
            }else{
                None
            }
        }
    }

    // zip: (1,2) (2,3) (3,4) (4,5)
    // map: 2,6,12,20
    // filter: 6,12
    // sum: 18
    let sum : u32 = Counter::new().zip(Counter::new().skip(1))
                    .map(|(a,b)|a*b)
                    .filter(|x| x % 3 == 0)
                    .sum();
    assert_eq!(sum,18);
}