#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (T, X): (usize, usize),
        A: [usize; T + 1],
    }

    let mut ans = vec![(0, A[0])];
    for (i, a) in A.iter().enumerate().skip(1) {
        if ans.last().unwrap().1.abs_diff(*a) >= X {
            ans.push((i, *a));
        }
    }

    for (i, a) in ans.iter() {
        println!("{i} {a}");
    }
}
