fn main() {
    let mut s = String::new();

    let data = "hello world";
    let s = data.to_string();
    let s = "hello world".to_string();
    let s = String::from("hello world");
    
    let mut s1 = String::from("c++ and ");
    let s2 = "rust";
    s1.push_str(s2);
    println!("s1 = {}", s1); // c++ and rust

    let s3 = String::from("hello ");
    let s4 = String::from("world");
    // let s5 = s3 + &s4; // incorrect! borrow of moved value: `s3`

    let s6 = String::from("abc");
    let s7 = String::from("def");
    let s8 = String::from("123");
    let s9 = s6 + "-" + &s7 + "-" + &s8;
    println!("{}", s9); // abc-def-123

    let s6_1 = String::from("abc");
    let s7_1 = String::from("def");
    let s8_1 = String::from("123");
    let s9_1 = format!("{s6_1}:{s7_1}:{s8_1}");
    println!("{}", s9_1); // abc:def:123

    let s10 = String::from("hello");
    // let c = s10[0]; // incorrect! string indices are ranges of `usize`

    let hello = "hello";
    let slice = &hello[0..3];
    println!("{}", slice); // hel

    for c in "hello".chars() {
        println!("{c}");
    }
}
