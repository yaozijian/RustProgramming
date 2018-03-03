
mod no_generics;
mod generics_in_fn;
mod generics_in_struct;
mod traitsec;
mod exercise1;
mod lifetime;

fn main() {
	no_generics::find_largest();
	generics_in_fn::find_largest();
	generics_in_struct::demo();

	traitsec::demo1();
	traitsec::demo2();

	exercise1::demo();

	lifetime::demo1();
  lifetime::demo2();
  lifetime::demo3("a","b");
}
