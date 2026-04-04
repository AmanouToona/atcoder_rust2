#![allow(non_snake_case)]
use ac_library::{Min, ModInt998244353 as Mint};
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [Mint; N],
        B: [Mint; M],
    }

    // (i - j).abs() のループ？
    let mut ans = Mint::new(0);
    for d in 0..10usize.pow(6) {
        let tmp = Mint::new(0);
        for i in 0..N {}
    }

    let suma: Mint = A.iter().sum();
    println!("{suma}");
}
