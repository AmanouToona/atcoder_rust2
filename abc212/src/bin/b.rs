#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        X: Chars,
    }
    let X: Vec<usize> = X.iter().map(|x| x.to_digit(10).unwrap() as usize).collect();

    if X.iter().skip(1).all(|x| *x == X[0]) {
        println!("Weak");
        return;
    }

    if X.iter()
        .take(3)
        .zip(X.iter().skip(1))
        .all(|(&x1, &x2)| (x1 + 1) % 10 == x2)
    {
        println!("Weak");
        return;
    }
    println!("Strong");
}
