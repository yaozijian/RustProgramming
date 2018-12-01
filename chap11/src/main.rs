#![allow(dead_code)]

extern crate chap11;

fn main() {
    println!("{}", chap11::add_two(3, 4));
	println!("{}", chap11::math::sub_two(3, 4));
}
