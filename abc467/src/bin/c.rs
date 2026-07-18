#![allow(non_snake_case)]
use proconio::input;

/*
直前が何で終わっているか？で場合ワケした DP?
*/

fn main() {
    input! {
        (N, _): (usize, usize),
        A: [usize; N],
        B: [usize; N - 1],
    }

    let mut pre0 = 0;
    let mut pre1 = 0;
    if A[0] == 1 {
        pre0 += 1;
    } else {
        pre1 += 1;
    }
    // eprintln!(":: {pre0} {pre1}");

    for (&a, &b) in A.iter().skip(1).zip(B.iter()) {
        // a を 0
        let nxt0 = if a == 0 {
            if b == 1 {
                pre1
            } else {
                pre0
            }
        } else {
            if b == 1 {
                pre1 + 1
            } else {
                pre0 + 1
            }
        };

        // a を 1
        let nxt1 = if a == 0 {
            if b == 1 {
                pre0 + 1
            } else {
                pre1 + 1
            }
        } else {
            if b == 1 {
                pre0
            } else {
                pre1
            }
        };

        pre0 = nxt0;
        pre1 = nxt1;

        // eprintln!(": {pre0} {pre1}");
    }

    let ans = pre0.min(pre1);
    println!("{ans}");
}
