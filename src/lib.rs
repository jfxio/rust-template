use std::iter::Iterator;

pub fn noisy_fold<'a>(
    items: impl Iterator<Item = &'a String>, 
    init: i32, 
    f: fn(i32, i32) -> i32
) -> i32 {
    let mut accumulator = init;

    for v in items {
        let a = v.parse::<i32>().expect(&format!("\"{v}\" is not an integer"));
        let new = f(accumulator, a);
        println!("  {accumulator} {a} => {new}");
        accumulator = new
    }

    return accumulator;    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noisy_fold_works() {
        let data: Vec<String> = ["5", "10", "15"].iter().map(|&s| s.to_string()).collect();
        assert_eq!(20, noisy_fold(data.iter(), -10, |a, b| a + b));
        assert_eq!(-750, noisy_fold(data.iter(), -1, |a, b| a * b));
    }

    // Size and layout of datatypes
    //     let _a = "foo";                // 16 bytes
    //     let _b = ['a', 'b', 'c'];      // 12 bytes sizeof char * 3     
    //     let _c = "bar".to_string();    // 24 bytes
    //     let _d: &str  = &_c;           // 16 bytes
    //     let _e = _c;                   // 24 bytes ala Vec<u8>
    //     let _f = &("baz".to_string()); // 8 bytes
    //     let _g = &_b;                  // 8 bytes
    //     let _h = &"foobar"[1..2];      // 16 bytes

    //     let _v1: Vec<i32> = vec!(1,2,3);        // 24 bytes

    //     // Vec<T,A> {
    //     //      buf: RawVect<T,A> {
    //     //          inner: RawVecInner<A> {
    //     //              ptr: Unique<u8>         // 8 bytes
    //     //              cap: Cap                // 8 bytes
    //     //              alloc: A
    //     //          }
    //     //          _marker: PhantomData<T>     // 0 bytes
    //     //      }
    //     //      len: usize                      // 8 bytes
    //     // }

    //     let _i1: usize = 10;     // 8 bytes
    //     let _i2 = 20;            // 4 bytes
    //     let _i3: u64 = 30;       // 8 bytes
    //     let _i4 = &30;           // 8 bytes

    //     let _c1: char = 'f';     // 4 bytes

}

