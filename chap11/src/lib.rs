
pub fn add_two(a: i32, b: i32) -> i32 {
	a + b
}

pub mod math;

#[cfg(test)]
mod test{

	use super::*;

	#[test]
	fn internal() {
		println!("这是单元测试");
		let (a, b) = setup();
		assert_eq!(add_two(a, b), a + b);
	}

	#[test]
	fn internal2() {
		println!("这是单元测试2");
		let (a, b) = setup();
		assert_eq!(math::sub_two(a, b), a - b);
	}

	fn setup() -> (i32, i32) {
		(3, 4)
	}
}