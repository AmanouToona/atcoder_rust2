#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        mut C: [usize; M],
        AB: [(usize, usize); N],
    }

    let mut ans = 0;
    for &(a, b) in AB.iter() {
        let a = a - 1;
        ans += C[a].min(b);

        C[a] = C[a].saturating_sub(b);
    }

    println!("{ans}");
}
