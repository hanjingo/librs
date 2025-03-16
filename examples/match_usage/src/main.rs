fn main() {
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

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            other => None,
            _ => None, // same as: other => None,
        }
    }
    let five = Some(5);
    let six = plus_one(five);  // match Some(i)
    let none = plus_one(None); // match other
}
