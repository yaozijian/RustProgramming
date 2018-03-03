
trait Flyable{
	fn fly(&self,src: String,dst: String);
}

trait Weapon{
	fn kill(&self,enmy: String);
}

//-----------------------------

struct Device<T>{
	tool: T
}

impl<T> Device<T>{
	fn new(item: T)->Device<T>{
		Device{
			tool: item,
		}
	}
}

// 为实现了Weapon特性的类型实现Fire方法
impl<T:Weapon> Device<T>{
	fn fire(&self,enmy: String){
		self.tool.kill(enmy);
	}
}

impl<T> Device<T> where T:Flyable{
	fn trasmit(&self,src: String,dst:String){
		self.tool.fly(src,dst);
	}
}

//-----------------------------

trait AirAttack{
	fn bomb(&self,base: String,dst:String,enmy: String);
}

// 总括实现
impl<T> AirAttack for T where T:Weapon + Flyable{
	fn bomb(&self,base:String,dst:String,enmy:String){
		self.fly(base,dst);
		self.kill(enmy);
	}
}

//-----------------------------

struct Plane{}
struct BattlePlane{}

impl Flyable for Plane{
	fn fly(&self,src:String,dst:String){
		println!("民用飞机装载乘客从{}飞往{}",src,dst);
	}
}

impl Flyable for BattlePlane{
	fn fly(&self,src:String,dst:String){
		println!("战斗机装载军人从{}飞往{}",src,dst);
	}
}

impl Weapon for BattlePlane{
	fn kill(&self,enmy:String){
		println!("战斗机消灭敌人{}",enmy);
	}
}

//=============================================================

pub fn demo(){

	let p1 = Plane{};
	let d1 = Device::new(p1);

	let p2 = BattlePlane{};
	let d2 = Device::new(p2);

	d1.trasmit(String::from("深圳"),String::from("武汉"));
	d2.trasmit(String::from("武汉"),String::from("深圳"));

	//编译时d1被单态化为Device_Plane,从而能够发现Plane类型没有实现kill方法
	//从而Device_Plane类型没有实现fire方法,下面对fire的调用是不正确的
	//d1.fire(String::from("希特勒"));
	d2.fire(String::from("匈奴"));

	//上面已经有为实现了Flyable和Weapon的类型实现AirAttack的总括实现,
	//所以这里可以调用AirAttack的bomb方法
	let d2 = BattlePlane{};
	d2.bomb(String::from("长安"),String::from("塞北"),String::from("匈奴"));

	//因为上面的总括实现,这里的类型转换也是正确的
	let d2 = &d2 as &AirAttack;
	d2.bomb(String::from("长安"),String::from("塞北"),String::from("匈奴"));
}

