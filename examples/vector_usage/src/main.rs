fn main() {
    let v:Vec<i32> = Vec::new();
    let v=vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("v[2]={third}"); // 3
    let second: Option<&i32> = v.get(1); // 2
    match second {
        Some(second) => println!("the second element is {second}."),
        None => println!("there is no second element!"),
    }
    //let not_exist = &v[100]; // incorrect! panic! index out of bounds
    let not_exist: Option<&i32> = v.get(100); // not panic!

    let mut v = vec![1, 2, 3];
    let first: &i32 = &v[0];
    // v.push(4); // incorrect! panic! cannot borrow `v` as mutable because it is also borrowed as immutable
    for e in &mut v {
        println!("{e}");
    }

    enum my_enum {
        Int(i32),
        Float(f64),
        Text(String),
    };
    let row = vec![
        my_enum::Int(3),
        my_enum::Text(String::from("blue")),
        my_enum::Float(123.456),
    ];
}
