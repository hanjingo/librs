fn main() {
    let num = 3;
    if num < 5 {
        println!("num < 5");
    } else if num > 5 && num < 10 {
        println!("num > 5 && num < 10");
    } else {
        println!("num > 10")
    }

    if num { // incorrect! the condition isn't a bool
        println!("incorrect")
    }

    if num == 3 { // correct
        println!("true")
    }

    let ret1 = if num > 5 { true } else { false };   // correct, ret1 = false
    let ret2 = if num > 5 { true } else { "hello" }; // incorrect!
}
