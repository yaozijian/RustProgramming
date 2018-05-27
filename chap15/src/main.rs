mod section1;
mod section4;
mod section5;
mod section6;

fn main() {
	section1::demo();
	section1::demo2();

	section4::demo();
	section4::demo2();

	section5::demo1();

	section6::ref_loop_demo();
	section6::weak_ref_demo();
}

