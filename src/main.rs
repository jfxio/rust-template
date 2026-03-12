//! The binary crate, `main`, which takes its arguments and noisily folds the sum and product.
//! This binary crate documentation apparently gets clobbered by the library crate documentation.
//! 

use std::env;
use math::noisy_fold;

fn main() {
    println!("Hello, world!");
    println!("This is the math binary (the name comes from Cargo.html)");

    let args: Vec<String> = env::args().skip(1).collect();
    let sum = noisy_fold(args.iter(), 0, |s, n| s+n);
    println!("sum = {sum}");
    let product = noisy_fold(args.iter(), 1, |p, n| p * n);
    println!("product = {product}")
}
