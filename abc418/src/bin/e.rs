#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;

fn euclid(x: i64, y: i64) -> i64 {
    if y == 0 {
        return x;
    }
    if x % y == 0 {
        return y;
    }
    euclid(y, x % y)
}

fn main() {
    input! {
        N: usize,
        xy: [(i64, i64); N],
    }

    let mut tilt: HashMap<(i64, i64), usize> = HashMap::new();
    let mut parallel: HashMap<(i64, i64, i64), usize> = HashMap::new();

    for i in 0..N {
        for j in (i..N).skip(1) {
            let (mut x_diff, mut y_diff) = (xy[i].0 - xy[j].0, xy[i].1 - xy[j].1);

            if x_diff == 0 {
                y_diff = y_diff.abs();
            }
            if y_diff == 0 {
                x_diff = x_diff.abs();
            }

            let e = euclid(x_diff.abs(), y_diff.abs());
            let (mut x, mut y) = (x_diff / e, y_diff / e);

            if x < 0 {
                x *= -1;
                y *= -1;
            }

            let x_time = if x_diff == 0 { 1 } else { x_diff.abs() / x };
            let y_time = if y_diff == 0 { 1 } else { y_diff.abs() / y };

            *tilt.entry((x, y)).or_default() += 1;
            *parallel.entry((x, y, x_time * y_time)).or_default() += 1;
        }
    }

    let mut ans = 0;
    for &v in tilt.values() {
        ans += v * (v - 1) / 2;
    }

    let mut sub = 0;
    for &v in parallel.values() {
        sub += v * (v - 1) / 2;
    }

    ans -= sub / 2;

    println!("{ans}");
}
