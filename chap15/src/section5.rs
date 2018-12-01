use std::cell::RefCell;

use std::fmt::Display;

pub trait Messager {
	fn send(&self, msg: &str);
}

struct MockMessenger {
	sent_messages: RefCell<Vec<String>>,
	//sent_messages: Vec<String>,
}

impl MockMessenger {
	fn new() -> MockMessenger {
		MockMessenger { sent_messages: RefCell::new(vec![]) }
	}

	fn demo(&self) {
		let mut one_borrow = self.sent_messages.borrow_mut();
		//let mut two_borrow = self.sent_messages.borrow_mut();

		one_borrow.push(String::from("abc"));
		//two_borrow.push(String::from("123"));
	}
}

impl Messager for MockMessenger {
	fn send(&self, message: &str) {
		// 错误：没法通过不可变的self引用来取得self.sent_messages的可变引用(push方法要求可变引用)
		//self.sent_messages.push(String::from(message));
		self.sent_messages.borrow_mut().push(String::from(message));
	}
}

pub fn demo1() {
	let obj = MockMessenger::new();
	obj.demo();
}

