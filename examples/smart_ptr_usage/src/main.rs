use crate::List::{Cons, Nil};
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
struct MyBox<T>(T);
impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct CustomSmartPointer {
  data: String,
}
impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  
  	let x = 5;
  	let y = &x;
  	assert_eq!(5, *y);
  
  	let y = Box::new(x);
  	assert_eq!(5, *y);
  
  	let y = MyBox::new(x);
  	assert_eq!(5, *y);
  
  	let c = CustomSmartPointer{
    	data: String::from("some data"),  
  	};
  	println!("CustomSmartPointer created.");
//  	c.drop(); // incorrect! explicit destructor calls not allowed
  	std::mem::drop(c);
  	println!("CustomSmartPointer dropped before the end of main.");
}