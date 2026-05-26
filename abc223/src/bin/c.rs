#![allow(non_snake_case)]
use proconio::input;

/*
導火線の燃焼時間合計の最大値は、 1o ** 3 * 10**3 * 10 ** 5 = 10 ** 11
*/

fn main() {
    input! {
        N: usize,
        ab: [(f64, f64); N],
    }

    let mut tot_time = 0.;
    for &(a, b) in ab.iter() {
        tot_time += a / b;
    }

    let mut lower = 0.;
    let mut larger = ab.iter().map(|x| x.0).sum::<f64>();

    while larger - lower > 0.00001 {
        let mid = (larger + lower) / 2.;

        let mut t = 0.;
        let mut res = mid;

        for &(a, b) in ab.iter() {
            if a <= res {
                t += a / b;
                res -= a;
            } else {
                t += res / b;
                break;
            }
        }

        if t * 2. >= tot_time {
            larger = mid;
        } else {
            lower = mid;
        }
    }
    println!("{larger}");
}
