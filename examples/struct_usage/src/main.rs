fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("def"),
        sign_in_count: 1,
    };
    user1.username = String::from("new user1");
    println!("{}", user1.username);
    
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email, // the field name same as parameter email, init shorthand!
            sign_in_count: 1,
        }
    }
    let user2 = build_user(String::from("hello"), String::from("user2"));

    let user3 = User {
        username: String::from("user3"),
        ..user1 // after that, we can't use user1 fields(active, email, sign_in_count) any more, because this fields has been moved
    };
    println!("{}", user1.username); // ok
    // println!("{}", user1.email);    // incorrect, value has been moved

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0); // black and origin values are different types because they’re instances of different tuple structs. 

    struct UnitLikeStruct;
    let obj = UnitLikeStruct;

    #[derive(Debug)] // print out debugging information
    struct Rectangle {
        width: u32,
        height: u32,
    };
    let rec1 = Rectangle {
        width: 30,
        height: 30,
    };
    println!("{rec1:?}");  // flat style
    println!("{rec1:#?}"); // another style

    let rec2 = Rectangle {
        width: dbg!(3 * 10),
        height: 30,
    };
    dbg!(&rec2);

    impl Rectangle {
        fn area(&self) -> u32 { // associated functions
            self.width * self.height
        }
    };
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool { // associated functions
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    };
    let rec3 = Rectangle {
        width: 10,
        height: 10,
    };
    println!("rec3.area()={}", rec3.area());
    println!("rec1.can_hold(rec2)={}", rec1.can_hold(&rec2));
    println!("rec1.can_hold(rec3)={}", rec1.can_hold(&rec3));

    let rec4 = Rectangle::square(1);
    println!("{rec4:#?}");
}
