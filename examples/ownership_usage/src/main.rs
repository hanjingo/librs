fn main() {
    fn hello() { // s is not valid here; it's not yet declared
        let s = "hello"; // s is valid from this point forward
    } // this scope is now over, and s is no longer valid

    let s1 = String::from("hello");
    let s2 = s1; // shallow copy
    // println!("{s1}, s2={s2}"); // incorrect! the pointer of string has moved to s2

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("s1={s1}, s2={s2}");

    let x = 5; 
    let y = x;
    println!("x={x}, y={y}");

    fn takes_ownership(arg: String) { // arg comes into scope
        println!("{arg}");
    } // Here, args goes out of scope and `drop` is called. The backing memory is freed.
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{s}"); // incorrect, value borrowed here after move

    fn makes_copy(arg: i32) { // arg comes into scop
        println!("{arg}");
    } // Here, arg goes out of scope. Nothing special happens.
    let x = 5;
    makes_copy(x);

    fn gives_ownership() -> String { // will move its return value into the function that calls it
        let str = String::from("yours"); // str comes into scope
        str // str is returned and moves out to the calling function
    }
    let s1 = gives_ownership();

    fn takes_and_gives_back(arg: String) -> String { // arg comes into scope
        arg // arg is returned and moves out to the calling function
    }
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
  
    fn change(s: &String) { // s is a reference to a String
    //   s.push_str(", world"); // incorrect! `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  	} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.
  	let s4 = String::from("hello");
  	change(&s4); // borrowing from s4
  	println!("s4={s4}");
  
  	fn change_mut(s: &mut String) {
      s.push_str(", world");
  	}
    let mut s5 = String::from("hello");
  	change_mut(&mut s5);
  	println!("s5={s5}");

    let mut s6 = String::from("hello");
    let r1 = &s6;
    let r2 = &s6; 
    // let r3 = &mut s6; // incorrect! We cannot have a mutable reference while we have an immutable one to the same value.
    // println!("{}, {}, and {}", r1, r2, r3); 

    let mut s7 = String::from("hello");
    let r1 = &s7; // ok
    let r2 = &s7; // ok
	println!("{r1}, {r2}"); 
	// variables r1 and r2 will not be used after this point.

	let r3 = &mut s7; // ok
	println!("{r3}");

    // fn dangle() -> &String { // incorrect! this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    //     let s = String::from("hello");
    //     &s // returns a reference to data owned by the current function
    // } // Here, s goes out of scope, and is dropped. Its memory goes away; Danger!

    let s8 = String::from("hello");
    println!("{}", &s8[0..2]); 					// he
    println!("{}", &s8[..2]);  					// he
    println!("{}", &s8[3..s8.len()]);   // lo
    println!("{}", &s8[3..]);           // lo
    println!("{}", &s8[0..s8.len()]);		// hello
    println!("{}", &s8[..]);						// hello
  	fn find_word(arg: &String) -> &str {
  		&arg[0..2]
  	}
  	let ret = find_word(&s8);
  	// s8.clear(); // incorrect! ret mutable borrow from s8. Rust disallows the mutable reference in `clear`.
  	println!("{ret}");
}