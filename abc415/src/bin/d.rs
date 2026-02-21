#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        mut AB: [(usize, usize); M],
    }

    AB.sort_by(|&x, &y| (x.0 - x.1).cmp(&(y.0 - y.1)));

    let mut ans = 0;
    let mut n = N;
    for &(a, b) in AB.iter() {
        if n < a {
            continue;
        }

        let y = (n - (a - 1)).div_ceil(a - b);
        ans += y;
        n -= y * (a - b);
    }

    println!("{ans}");
}
