#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (H, W): (usize, usize),
    }

    let mut ans = vec![vec!['.'; W]; H];

    for (h, ans) in ans.iter_mut().enumerate() {
        for (w, ans) in ans.iter_mut().enumerate() {
            if h == 0 || h == H - 1 || w == 0 || w == W - 1 {
                *ans = '#';
            }
        }
    }

    for i in ans.iter() {
        let a = i.iter().join("");
        println!("{a}");
    }
}
