#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        rc: [(usize, usize); M],
    }

    let mut use_r = vec![false; N + 1];
    let mut use_c = vec![false; N + 1];
    let mut ans = 0;
    for &(r, c) in rc.iter().rev() {
        if !use_r[r] && !use_c[c] {
            ans += 1;
        }
        use_c[c] = true;
        use_r[r] = true;
    }
    println!("{ans}");
}
