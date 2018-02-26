
use std::collections::HashMap;

pub fn hashmap_demo_1(){

	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"),10);
	scores.insert(String::from("Yellow"),50);

	let r = String::from("red");
	scores.insert(r,30);

	let teams  = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];
	let fenshu: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
	for (k,v) in fenshu{
		println!("{} -> {}",k,v);
	}

	// HashMap::get()方法返回Option<T>类型，不会panic
	match scores.get("green"){
		Some(x) => println!("green --> {}",x),
		None => println!("not contain green"),
	}

	// 通过下标访问不存在的元素时会panic
	println!("red -> {}",scores["red"]);

	// 上面的insert()方法会让HashMap获取s的所有权,这里s已经无效了,不能使用
	//scores.insert(r,40);
	scores.insert(String::from("red"),40);
	println!("red -> {}",scores["red"]);

	scores.entry(String::from("green")).or_insert(80);
	scores.entry("red".to_string()).or_insert(1);
}

pub fn hashmap_demo_2(){

	let text = "hello world wonderfull world";
	let mut map = HashMap::new();

	for word in text.split_whitespace(){
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}

	for (k,v) in map{
		println!("{} -> {}",k,v);
	}
}