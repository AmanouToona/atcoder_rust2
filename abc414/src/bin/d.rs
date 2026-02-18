#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        mut X: [usize; N],
    }
    X.sort();
    X.dedup();

    let mut sub: Vec<usize> = X
        .iter()
        .zip(X.iter().skip(1))
        .map(|(x1, x2)| x2 - x1)
        .collect();
    sub.sort();

    let ans: usize = sub.iter().take(sub.len().saturating_sub(M - 1)).sum();
    println!("{ans}");
    eprintln!("{:?}", sub);
}
