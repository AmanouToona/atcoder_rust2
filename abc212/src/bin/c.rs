#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        mut A: [usize; N],
        mut B: [usize; M],
    }
    A.sort();
    B.sort();

    let mut ans = usize::MAX;
    // a <= b
    let mut i = 0;
    for a in A.iter() {
        while i < B.len() && B[i] < *a {
            i += 1;
        }

        if i >= B.len() {
            break;
        }
        ans = ans.min(B[i] - a);
    }

    // b < a
    i = 0;
    for &b in B.iter() {
        while i < A.len() && A[i] <= b {
            i += 1;
        }
        if i >= A.len() {
            break;
        }
        ans = ans.min(A[i] - b);
    }
    println!("{ans}");
}
