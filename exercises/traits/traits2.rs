// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.

// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

//

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {// 这里添加 `mut` 使初入的参数可变居然也是可以的，
        // 一般情况下，当我们为某个类型实现 trait 时，我们确实需要遵循 trait 定义的方法签名。
        // 这意味着我们不能更改方法的参数类型或返回类型。
        // 然而，在这种情况下，方法签名实际上并没有真正改变。
        // 在 trait 定义中，`append_bar` 方法接受 `self` 并返回 `Self` 类型。这意味着它接受调用者的所有权并返回相同的类型。
        // 在 impl 块中,虽然您添加了 `mut` 关键字，但 `self` 的类型 (`Vec<String>`) 和返回类型 (`Self`, 即 `Vec<String>`) 都没有改变。
        // `mut` 只是允许您在方法体内部修改 `self`，但它不改变 `self` 的类型。
        // 所以，在这种情况下，即使您在实现方法时添加了 `mut` 关键字，方法的签名实际上仍然与 trait 定义中的一致。

        // 在类型系统中，*mut T* 和 *T* 是一个东西吗？
        // 在 Rust 类型系统中，`mut T` 和 `T` 不是相同的。但在方法参数中，`mut` 是一个模式而不是类型的一部分。它表明参数是可变的。在类型签名中，`self`, `&self`, 和 `&mut self` 是特殊情况。
        //   - `self` 表示方法获取了调用者的所有权。
        //   - `&self` 表示方法获取了调用者的不可变引用。
        //   - `&mut self` 表示方法获取了调用者的可变引用。
        // 但是在方法体内，您可以选择是否将 `self` 声明为可变，就像您在 `fn append_bar(mut self) -> Self` 中做的那样。这不会影响方法的签名，因为它只影响方法体内的 `self` 的可变性。
        // 简而言之：
        //   - 在类型系统中，`mut T` 和 `T` 是不同的。
        //   - 在方法签名中，`mut` 关键字不影响方法的签名。它只影响方法体内的参数的可变性。
  

        // 如果不加 `mut` 则👇三行也可以实现
        // let mut result = self.clone();
        // result.push("Bar".to_string());
        // result

        self.push("Bar".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
