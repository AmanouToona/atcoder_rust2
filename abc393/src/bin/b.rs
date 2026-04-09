#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut ans = 0;
    for i in 0..S.len() {
        for j in i + 1..S.len() {
            for k in j + 1..S.len() {
                if k - j != j - i {
                    continue;
                }
                if S[i] == 'A' && S[j] == 'B' && S[k] == 'C' {
                    ans += 1;
                }
            }
        }
    }
    println!("{ans}");
}
