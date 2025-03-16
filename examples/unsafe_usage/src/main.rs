extern "C" {
    fn abs(input: i32) -> i32;
  }
  
  #[no_mangle]
  pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
  }
  
  unsafe trait Foo {}
  unsafe impl Foo for i32 {}
  
  fn add_one(x: i32) -> i32 {
    x + 1
  }
  
  fn do_twice(f: fn(i32)->i32, arg: i32) -> i32 {
    f(arg) + f(arg)
  }
  
  fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
  }
  
  #[some_attribute]
  pub fn some_name(input: TokenStream) -> TokenStream {}
  
  fn main() {
    // unsafe
    unsafe fn dangerous() {}
    unsafe {
      dangerous(); // incorrect!
    }
    unsafe {
      println!("Absolute value of -3 according to C:{}", abs(-3));
    }
    
    // function pointer
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");
  }