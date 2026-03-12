use math::noisy_fold;
use std::env;

fn main() {
    println!("This this is the sum binary crate.");

    let args: Vec<String> = env::args().skip(1).collect();
    let sum = noisy_fold(args.iter(), 0, |s, n| s + n);
    println!("  sum = {sum}");
}
