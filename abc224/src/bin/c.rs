#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

/*
外積 != 0 ならば良い
*/
fn main() {
    input! {
        N: usize,
        xy: [(i64, i64); N],
    }

    let mut ans = 0;
    for i in (0..N).combinations(3) {
        let a = xy[i[0]];
        let b = xy[i[1]];
        let c = xy[i[2]];

        let ab = (b.0 - a.0, b.1 - a.1);
        let ac = (c.0 - a.0, c.1 - a.1);

        if ab.0 * ac.1 - ac.0 * ab.1 != 0 {
            ans += 1;
        }
    }
    println!("{ans}");
}
