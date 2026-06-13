#![allow(non_snake_case)]
use proconio::input;

fn calc_cost(a: usize, b: usize, x: usize, y: usize) -> usize {
    let mut cost = 0;

    cost += a.min(b) * x.min(y) * 2;

    if x >= y {
        let res = x - y;
        cost += (res / 2) * (a + b) + (res % 2) * a;
    } else {
        let res = y - x;
        cost += (res / 2) * (a + b) + (res % 2) * b;
    }

    cost
}

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            (A, B, X, Y): (usize, usize, i64, i64),
        }
        let x = X.unsigned_abs() as usize;
        let y = Y.unsigned_abs() as usize;

        if x >= y {
            let mut min = 0;
            let mut max = ((x + 2) - y) / 2;

            while max - min > 1 {
                let mid = (min + max) / 2;

                if calc_cost(A, B, x, y + min * 2) < calc_cost(A, B, x, y + mid * 2) {
                    max = mid;
                } else {
                    min = mid;
                }
            }

            let ans = calc_cost(A, B, x, y + min * 2);
            println!("{ans}");
        } else {
            let mut min = 0;
            let mut max = ((y + 2) - x) / 2;

            while max - min > 1 {
                let mid = (min + max) / 2;

                if calc_cost(A, B, x + min * 2, y) < calc_cost(A, B, x + mid * 2, y) {
                    max = mid;
                } else {
                    min = mid;
                }
            }

            let ans = calc_cost(A, B, x + min * 2, y);
            println!("{ans}");
        }
    }
}
