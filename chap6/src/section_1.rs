pub fn enum_demo1(){

    #[derive(Debug)]
    enum IpAddrKind{
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr{
        kind:IpAddrKind,
        addr:String,
    }

    let loopback = IpAddr{
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
    let home = IpAddr{
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };

    println!("loopback={:#?}",loopback);
    println!("home={:#?}",home);
}

pub fn enum_demo2(){

    #[derive(Debug)]
    enum IpAddr{
        V4(u8,u8,u8,u8),
        V6(String),
    };

    let loopback = IpAddr::V4(127,0,0,1);
    let home = IpAddr::V6(String::from("::1"));
    println!("loopback={:#?}",loopback);
    println!("home={:#?}",home);
}

pub fn enum_demo3(){

    #[derive(Debug)]
    enum IpAddr{
        V4{a:u8,b:u8,c:u8,d:u8},
        V6(String),
    };

    let loopback = IpAddr::V4{a:127,b:0,c:0,d:1};
    let home = IpAddr::V6(String::from("::1"));
    println!("loopback={:#?}",loopback);
    println!("home={:#?}",home);
}

pub fn enum_demo4(){
    #[derive(Clone)]
    enum Message{
        Quit,
        Move{x: i32,y: i32},
        Write(String),
        Color(i32,i32,i32),
    }

    impl Message{
        fn show(self){
            match self{
                Message::Quit => println!("Quit"),
                Message::Move{x:x1,y:y1} => println!("Move to ({},{})",x1,y1),
                Message::Write(msg) => println!("Write msg: {}",msg),
                Message::Color(r,g,b) => println!("Set Color to ({},{},{})",r,g,b),
            }
        }
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
    }

    let msg = vec![
        Message::Quit,
        Message::Move{x: 1,y: 2},
        Message::Write(String::from("Console")),
        Message::Color(3,4,5),
    ];

    for i in &msg{
        i.show2();
    }

    for i in msg{
        i.show();
    }
}
