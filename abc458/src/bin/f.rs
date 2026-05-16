#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
fn main() {
    let t6 = 10usize.pow(6);
    let mut a = mint::new(5);
    println!("{}", a.pow(t6 as u64));
}
