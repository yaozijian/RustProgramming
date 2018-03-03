
use std::fmt::*;

#[allow(dead_code)]
struct Point<T> {
	x: T,
	y: T,
}

#[allow(dead_code)]
struct Point2<T, U> {
	x: T,
	y: U,
}

impl<T> Point<T> {
	fn _x(&self) -> &T {
		&self.x
	}
}

// 为特定类型定义一些方法
impl Point<f32> {
	fn _distance_to_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

// 为实现了某些特定特性的类型增加一些方法
impl<T: Display + PartialOrd> Point<T> {
	#[allow(dead_code)]
  fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
  }
}

// 结构体带泛型参数,结构体方法带有不同的泛型参数
impl<T, U> Point2<T, U> {
	fn _mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
		Point2 {
			x: self.x,
			y: other.y,
		}
	}
}

pub fn demo() {
	let _integer = Point { x: 5, y: 10 };
	let _float = Point { x: 1.0, y: 4.0 };
}
