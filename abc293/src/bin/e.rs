#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (A, X, M): (u128, u128, u128),
    }

    if A == 1 {
        let ans = X % M;
        println!("{ans}");
        return;
    }

    let modulo = (A - 1) * M;

    let mut A_double = [A % modulo; 40];
    for i in 1..A_double.len() {
        A_double[i] = A_double[i - 1] * A_double[i - 1];
        A_double[i] %= modulo;
    }

    let mut rn = 1;
    for (i, &d) in A_double.iter().enumerate() {
        if X >> i & 1 == 1 {
            rn *= d;
            rn %= modulo;
        }
    }

    let ans = ((rn + modulo - 1) % modulo) / (A - 1);
    println!("{ans}");
}
