#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        L: [usize; N],
    }

    let mut ans = usize::MAX;
    let mut left = 0;
    let tot: usize = L.iter().sum();

    for &l in L.iter() {
        left += l;
        ans = ans.min((tot - left).abs_diff(left));
    }

    println!("{ans}");
}
