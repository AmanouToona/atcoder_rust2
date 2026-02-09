#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, T): (usize, usize),
        mut A: [usize; N],
    }

    A.push(T);

    let mut close = vec![A[0]];
    let mut u = A[0];
    for &v in A.iter().skip(1) {
        if v - u < 100 {
            continue;
        }
        close.push(v);
        u = v;
    }
    close.push(T);

    let mut sum = 0;
    for (&a, &b) in close.iter().zip(close.iter().skip(1)) {
        sum += (b - a).min(100);
    }

    let ans = T - sum;
    println!("{ans}");
}
