// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



#[rustfmt::skip]
macro_rules! my_macro {
    
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };

    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}

// 你的宏定义试图支持两种形式：一种不带参数，另一种带一个表达式参数。然而，宏匹配是从上到下进行的，并且在第一个匹配的分支上停止。由于你的第一个分支能够匹配任何情况（包括没有参数的情况），所以第二个分支永远不会被触发。
// 为了解决这个问题，你应该将带参数的分支放在前面
// 这样，如果宏被传递了一个参数，第一个分支将被匹配；如果没有参数被传递，第二个分支将被匹配。