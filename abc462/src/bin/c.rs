#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        mut xy: [(usize, usize); N],
    }

    xy.sort_by_key(|x| x.0);
    // eprintln!("{:?}", xy);

    let mut ans = 0;
    let mut min_y = N + 1;
    for &(_, y) in xy.iter() {
        if y < min_y {
            min_y = y;
            ans += 1;
        }
    }
    println!("{ans}");
}
