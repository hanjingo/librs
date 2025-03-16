use std::thread;
use std::time::Duration;
use std::sync::mpsc;
fn main() {
  // spawn + join
  let handle = thread::spawn(||{
    for i in 1..10 {
      println!("hi number {i} from the spawned thread!");
      thread::sleep(Duration::from_millis(1));
    }
  });
  handle.join().unwrap();
  for i in 1..5 {
    println!("hi number {i} from the main thread!");
    thread::sleep(Duration::from_millis(1));
  }
  
  // channel
  let (tx, rx) = mpsc::channel(); // mpsc: multiple producer, single consumer
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });
  for received in rx {
    println!("Got: {received}");
  }
}