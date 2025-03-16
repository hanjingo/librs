fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
      if item > largest {
        largest = item;
      }
    }
    largest
}
pub trait Summary {
  fn summarize(&self)->String;
}
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}
impl Summary for NewsArticle {
  fn summarize(&self)->String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest number is {result}");
  
  	let char_list = vec!['y', 'm', 'a', 'q'];
  	let result = largest(&char_list);
    println!("the largest char is {result}");
}