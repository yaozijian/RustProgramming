fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

pub fn demo1() {
	let string1 = String::from("abcd");
	let string2 = "xyz";

	let result = longest(string1.as_str(), string2);
	println!("The longest string is {}", result);
}

pub fn demo2() {
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = "xyz";
    result = longest(string1.as_str(), string2);
  }
  println!("The longest string is {}", result);
}

#[allow(unused_variables)]
pub fn demo3(x:&str,y:&str) -> String{
  let result = String::from("really long string");
  result
}