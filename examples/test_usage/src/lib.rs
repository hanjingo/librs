// run: cargo new adder --lib

// lib.rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Guess {
    value: i32,
}
impl Guess{
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {value}.");
    }
    Guess{value}
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
      	assert_ne!(result, 1);
    }

    #[test]
    fn greeting_contains_name() {
        let result = "carol";
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result,
        );
    }
  
    #[test]
  	#[should_panic(expected = "less than or equal to 100")]
  	fn greater_than_100() {
    	Guess::new(200);  
  	}
}
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
// run: cargo test