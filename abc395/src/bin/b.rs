#![allow(non_snake_case)]
use itertools::Itertools;
use num::Integer;
use proconio::input;
fn main() {
    input! {
        N: usize,
    }

    let mut ans = vec![vec!['.'; N]; N];
    for i in 0..N {
        let j = N - 1 - i;
        if i > j {
            continue;
        }

        let color = if i.is_odd() { '.' } else { '#' };
        for ii in i..=j {
            for jj in i..=j {
                ans[ii][jj] = color;
            }
        }
    }

    for a in ans.iter() {
        let a: String = a.iter().join("");
        println!("{a}");
    }
}
