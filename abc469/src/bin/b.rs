#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let S: Vec<char> = ['x']
        .iter()
        .chain(S.iter())
        .chain(['x'].iter())
        .cloned()
        .collect();

    let mut ans = 0;
    for ((pre, now), nxt) in S.iter().zip(S.iter().skip(1)).zip(S.iter().skip(2)) {
        if *now == 'x' && *pre == 'x' && *nxt == 'x' {
            ans += 1
        }
    }

    println!("{ans}");
}
