pub fn demo() {
	enum List {
		Cons(i32, Box<List>),
		Nil,
	}
	let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
	let _b = List::Cons(3, Box::new(a));
	//编译错误：上个语句将变量a的所有权移动到b中了,a已经失效
	//let c = List::Cons(4, Box::new(a));
}

pub fn demo2() {
	use std::rc::Rc;
	enum List {
		Cons(i32, Rc<List>),
		Nil,
	}

	let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
	println!("a的引用计数：{}", Rc::strong_count(&a));//1

	let _b = List::Cons(3, Rc::clone(&a));
	println!("a的引用计数：{}", Rc::strong_count(&a));//2

	{
		let _c = List::Cons(4, Rc::clone(&a));
		println!("a的引用计数：{}", Rc::strong_count(&a));//3
	}

	println!("a的引用计数：{}", Rc::strong_count(&a));//2
}

