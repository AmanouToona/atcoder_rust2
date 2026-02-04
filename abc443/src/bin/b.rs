#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, K) : (usize, usize),
    }

    let mut ans = 0;
    let mut sum = 0;
    while sum < K {
        sum += ans + N;
        if sum >= K {
            break;
        }
        ans += 1;
    }

    println!("{ans}");
}
