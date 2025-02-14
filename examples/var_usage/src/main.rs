fn main() {
    // let mut sum = 0;
    // sum--;
    // sum++;

    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // same as: let b = [3, 3, 3, 3, 3]

    let arg1 = a[0];  // 1
    // let arg2 = a[10]; // incorrect;

    fn hello() -> usize { 4 };
    let idx = hello();
    let arg3 = a[idx]; // compiles successfully. Buf if idx >= 5, panic!

    let a: (i32, f64, u8) = (1, 2.0, 3);

    let b = (1, 2.0, 3);
    let (x, y, z) = b; // x = 1, y = 2.0, z = 3

    let arg1 = a.0; // 1
    let arg2 = a.1; // 2.0
    let arg3 = a.2; // 3
    println!("arg1={arg1}, arg2={arg2}, arg3={arg3}");

    let a = 'z';
    let b: char = 'ℤ';
    let c = '😻';

    let sum = 1 + 1; // 2, correct
    let sum = 1 - 1; // 0, correct
    let sum = 1 * 1; // 1, correct
    let sum = 1 / 1; // 1, correct
    let sum = 1 % 1; // 0, correct
    println!("sum={sum}");

    const DUR: u32 = 2 * 60;
}
