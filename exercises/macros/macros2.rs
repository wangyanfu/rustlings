// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}


// 本节考察 macro 的定义顺序，本地 macro 必须要在调用它的函数之前被定义（或者外部宏的引入）。这个和函数不一样；