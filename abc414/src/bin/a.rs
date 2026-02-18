#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, L, R): (usize, usize, usize),
        XY: [(usize, usize); N],
    }

    let mut ans = 0;
    for &(x, y) in XY.iter() {
        if x <= L && y >= R {
            ans += 1;
        }
    }
    println!("{ans}");
}
