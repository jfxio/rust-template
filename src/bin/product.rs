use std::env;
use math::noisy_fold;

fn main() {
    println!("This this is the product binary crate.");

    let args: Vec<String> = env::args().skip(1).collect();
    let product = noisy_fold(args.iter(), 1, |p, n| p * n);
    println!("  product = {product}")
}
