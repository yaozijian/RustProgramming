
#![allow(dead_code)]

extern crate chap11;

#[test]
fn add() {
	println!("这是集成测试add");
    let (a, b) = setup();
    assert_eq!(chap11::add_two(a, b), a + b);
}

#[test]
fn sub() {
	println!("这是集成测试sub");
	let (a, b) = setup();
	assert_eq!(chap11::math::sub_two(a, b), a - b);
}

fn setup() -> (i32, i32) {
	(3, 4)
}