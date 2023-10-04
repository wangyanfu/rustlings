
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



use std::f32;

fn main() {
    let pi = std::f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}

// Rust 会存储 Rust 标准库中任何长精度或无限精度数学常量的最高精度版本。
// 数学常量的最高精度版本。
// 我们可能会对某些数学常量使用自己的近似值、
// 但 clippy 会将这些不精确的数学常量视为潜在错误的来源。
// 潜在错误。