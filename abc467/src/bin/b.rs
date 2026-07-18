#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        abs: [(usize, usize, String); N],
    }

    let mut ans = 0;
    for (a, b, s) in abs {
        let s = s.as_str();
        match s {
            "keep" => {
                ans += b - a;
            }
            _ => {
                continue;
            }
        }
    }
    println!("{ans}");
}
