pub fn demo(){

  let x = 5;
  let y = &x;
  let z = Box::new(x);

  assert_eq!(x,5);
  assert_eq!(*y,5);
  assert_eq!(*z,5);
}

pub fn demo2(){

  struct MyBox<T>(T);

  impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
      MyBox(x)
    }
  }

  use std::ops::Deref;

  impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T{
      &self.0
    }
  }

  let x = 5;
  let y = &x;
  let z = MyBox::new(x);
  let a = MyBox::new(String::from("ABC"));

  fn hello(x : &str){ println!("{}",x); };

  assert_eq!(x,5);
  assert_eq!(*y,5);
  assert_eq!(*z,5);
  assert_eq!(*a,String::from("ABC"));

  hello(a.deref().deref());
  hello(&a);
  hello(&*a);
}
