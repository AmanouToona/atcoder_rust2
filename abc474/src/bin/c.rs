#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        P: [usize; N],
        A: [usize; Q],
    }

    let mut used = vec![false; N + 1];
    let mut ans = vec![0; N + 1];

    let mut i = N;
    for &a in A.iter().rev() {
        if !used[a] {
            used[a] = true;
            ans[i] = a;
            i -= 1;
        }
    }

    let mut i = 0;
    for &p in P.iter() {
        if !used[p] {
            ans[i + 1] = p;
            i += 1;
        }
    }

    let ans: String = ans.iter().skip(1).join(" ");
    println!("{ans}")
}
