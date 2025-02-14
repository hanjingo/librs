fn main() {
    enum IP {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IP::V4(127, 0, 0, 1);
    let loopback = IP::V6(String::from("::1"));

    enum Message {
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn Call(&self) {
        }
    }
    let msg1 = Message::Write(String::from("hello"));
    msg1.Call();

    let some1 = Option::Some(5); // Option prefix is not a must
    let some2 = Some('e');
    let some3 : Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // incorrect! the trait `Add<Option<i8>>` is not implemented for `i8`

    enum Coin {
        Penny,
        Nickel,
    }
    fn match_coin(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("match Penny");
                1
            }
            Coin::Nickel => 5,
        }
    }
    println!("{}", match_coin(Coin::Penny));
    println!("{}", match_coin(Coin::Nickel));
}
