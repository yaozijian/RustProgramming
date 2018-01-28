fn main() {
    println!("area1={}",area1(1,2));
    println!("area2={}",area2((3,4)));
    println!("area3={}",area3(&Rect{width: 5,height: 6}));
    let r = &Rect{width:7,height:8};
    println!("area4={}",Rect::area4(r));
    println!("area5={}",r.area5());
}

fn area1(w: u32,h: u32) -> u32 { w * h}
fn area2(wh:(u32,u32)) -> u32 { wh.0 * wh.1 }

struct Rect{
    width: u32,
    height: u32,
}

fn area3(r : &Rect) -> u32{ r.width * r.height }

impl Rect{
    fn area4(r : &Rect) -> u32{ r.width * r.height }
    fn area5(&self) -> u32 {self.width * self.height}
}