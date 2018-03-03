
fn largest<T>(list: &[T]) -> T where T: PartialOrd + Copy {
	let mut val = list[0];
	for &item in list.iter() {
		if item > val {
			val = item;
		}
	}
	val
}

pub fn find_largest() {
	let number_list = vec![34, 50, 25, 100, 65];

	let result = largest(&number_list);
	println!("The largest number is {}", result);
	assert_eq!(result, 100);

	let char_list = vec!['y', 'm', 'a', 'q'];

	let result = largest(&char_list);
	println!("The largest char is {}", result);
	assert_eq!(result, 'y');
}
