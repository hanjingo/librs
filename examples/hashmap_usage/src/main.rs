use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("key1"), 1);
    scores.insert(String::from("key2"), 2);
    let key = String::from("key1");
    let value = scores.get(&key).copied().unwrap_or(0);
    println!("{}", value); // 1

    scores.insert(String::from("key1"), 3); // 3, 2
    scores.entry(String::from("key2")).or_insert(4); // 3, 2
    scores.entry(String::from("key3")).or_insert(5); // 3, 2, 5
    for (k, v) in &scores {
        println!("{k}:{v}"); // 3,2,5
    }

    let key4 = String::from("key4");
    let value4 = String::from("value4");
    let mut hm = HashMap::new();
    hm.insert(key4, value4);
    // println!("{}", key4); // incorrect! value borrowed here after move
}
