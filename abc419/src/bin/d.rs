#![allow(non_snake_case)]
use itertools::Itertools;
use num::Integer;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (N, M): (usize, usize),
        S: Chars,
        T: Chars,
        LR: [(usize, usize); M],
    }

    let mut parity = vec![0; N + 1];

    for &(l, r) in LR.iter() {
        let l = l - 1;
        parity[l] += 1;
        parity[r] -= 1;
    }

    for i in 0..N {
        parity[i + 1] += parity[i];
    }

    let mut ans = Vec::new();
    for (i, p) in parity.iter().enumerate().take(N) {
        if p.is_even() {
            ans.push(S[i]);
        } else {
            ans.push(T[i]);
        }
    }

    let ans: String = ans.iter().join("");
    println!("{ans}");
}
