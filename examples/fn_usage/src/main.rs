fn main() {
    let y = {
        let x = 3;
        x + 1 // NOTICE, there is not an ending semicolon!!!
    };
    println!("The value of y={y}"); // y=4

    fn hello(arg: i32) -> i32 { arg + 1 }
    let var1 = hello(5); // 6
    println!("var1={var1}");

    fn world(arg: i32) -> i32 { return arg + 1; }
    let var2 = world(5); // 6
    println!("var2={var2}");

    // fn err_func(arg: i32) -> i32 { arg + 1; }
    // let var3 = err_func(5); // incorrect! Nothing is returned

    // let x = (let y = 6); // there isn't anything for x to bind to!
}
