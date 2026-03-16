#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {N: i128}

    let mut ans: i128 = 0;
    let mut i = 1;
    while i <= N {
        let j = N / i;
        let nxt = N / j + 1;
        ans += j * (nxt - i);
        i = nxt;
    }
    println!("{ans}");
}
