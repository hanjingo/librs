fn main() {
    let mut counter = 0;
    let ret = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("ret = {ret}"); // 20

    let mut counter = 0;
    'counting: loop {
        let mut remain = 10;
        loop {
            if remain == 9 {
                break; // quiet inner loop
            }
            if counter == 2 {
                break 'counting; // quiet outter loop. all loops end
            }
            remain -= 1;
        }
        counter += 1;
    }
    println!("counter = {counter}"); // 2
}
