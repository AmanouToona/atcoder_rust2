#![allow(non_snake_case)]
use proconio::input;

fn calc_cost(a: usize, b: usize, x: usize, y: usize) -> usize {
    let mut cost = 0;

    cost += a.min(b) * x.min(y) * 2;

    let res = x.abs_diff(y);
    let even = res / 2;
    let odd = res.div_ceil(2);

    if x >= y {
        cost += a.min(3 * b) * odd + (b).min(a * 3) * even;
    } else {
        cost += a.min(3 * b) * even + b.min(a * 3) * odd;
    }
    cost
}

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            (a, b, X, Y): (usize, usize, i64, i64),
        }
        let x = X.unsigned_abs() as usize;
        let y = Y.unsigned_abs() as usize;

        let ans = calc_cost(a, b, x, y);

        println!("{ans}");
    }
}
