fn main() {
    mod outermost {
        pub fn f1() { println!("f1"); }
        fn f2() { println!("f2"); }
        mod inside {
          pub fn f3() { println!("f3"); }
          fn f4() { println!("f4"); }
        }
        pub mod outside {
            pub fn f5() { println!("f5"); }
            fn f6() { println!("f6"); }
        }
    }

    outermost::f1();
    // outermost::f2(); // incorrect! function `f2` is private
    // outermost::inside::f3(); // incorrect! private module
    // outermost::inside::f4(); // incorrect! private module
    outermost::outside::f5();
    // outermost::outside::f6(); // incorrect! function `f6` is private
}
