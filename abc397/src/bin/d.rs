#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {N: i128}

    // N = (x - y)(x**2 + xy + y**2)
    // d = x - y
    for d in 1..=10i128.pow(6) {
        if d.pow(3) > N {
            break;
        }

        let mut max: i128 = 10i128.pow(9) + 1;
        let mut min: i128 = 0;
        while max - min > 1 {
            let mid = (max + min) / 2;
            if N - d.pow(3) < 3 * d.pow(2) * mid + 3 * mid.pow(2) * d {
                max = mid;
            } else {
                min = mid;
            }
        }

        let y: i128 = min;
        if N - d.pow(3) == 3 * d.pow(2) * y + 3 * y.pow(2) * d && y > 0 {
            let x: i128 = d + y;

            println!("{x} {y}");
            return;
        }
    }

    println!("-1");
}
