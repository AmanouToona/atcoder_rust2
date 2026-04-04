#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (H, W): (usize, usize),
    }

    let mut ans = vec![vec!['.'; W]; H];

    for h in 0..H {
        for w in 0..W {
            if h == 0 || h == H - 1 {
                ans[h][w] = '#';
            } else if w == 0 || w == W - 1 {
                ans[h][w] = '#';
            }
        }
    }

    for i in ans.iter() {
        let a = i.iter().join("");
        println!("{a}");
    }
}
