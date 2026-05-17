#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (H, W): (usize, usize),
    }

    let mut ans = vec![vec![0; W]; H];
    for h in 0..H {
        for w in 0..W {
            for &(dh, dw) in [(0, 1), (!0, 0), (0, !0), (1, 0)].iter() {
                let vh = h.wrapping_add(dh);
                let vw = w.wrapping_add(dw);

                if vh < H && vw < W {
                    ans[vh][vw] += 1;
                }
            }
        }
    }

    for i in ans.iter() {
        let i: String = i.iter().join(" ");
        println!("{i}");
    }
}
