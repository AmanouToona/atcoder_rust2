#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        XY: [(usize, usize); Q],
    }

    let mut ver = vec![1; N];

    let mut oldest = 0;
    for &(x, y) in XY.iter() {
        let x = x - 1;
        let y = y - 1;
        let mut ans = 0;
        for v in oldest..=x {
            ver[y] += ver[v];
            ans += ver[v];
            ver[v] = 0;
        }

        println!("{ans}");

        oldest = oldest.max(x);
    }
}
