// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DON

fn main() {
    let mut shopping_list: Vec<Box<dyn AsRef<str>>> = Vec::new();
    shopping_list.push(Box::new("milk"));
}

