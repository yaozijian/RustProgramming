fn main() {
    const N: i64 = 10;
    fib1(N);
    fib2(N);
    fib3(N);
}

fn fib1(n: i64) {
    let mut cur: (i64, i64) = (1, 1);
    println!("{}", cur.0);
    for _ in 1..n {
        println!("{}", cur.1);
        cur = (cur.1, cur.0 + cur.1);
    }
}

fn fib2(n: i64) {
    let mut cur = [1i64, 1i64];
    println!("{}", cur[0]);
    for _ in 1..n {
        println!("{}", cur[1]);
        cur = [cur[1], cur[0] + cur[1]];
    }
}

fn fib3(n: i64) -> (i64,i64){
    match n{
        2 => {
           println!("{}",1);
           println!("{}",1);
           (1,1)
        }
        x if x > 2 => {
            let t = fib3(x - 1);
            println!("{}",t.0 + t.1);
            (t.1,t.0 + t.1)
        }
        _ => (0,0)
    }
}
