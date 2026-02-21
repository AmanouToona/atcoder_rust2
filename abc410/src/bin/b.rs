#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        X: [usize; Q],
    }

    let mut b = vec![0; N];
    let mut ans = Vec::new();
    for &x in X.iter() {
        if x > 0 {
            b[x - 1] += 1;
            ans.push(x);
        } else {
            let mut min = usize::MAX;
            let mut min_pos = 0;

            for (i, b_in) in b.iter().enumerate() {
                if *b_in < min {
                    min = *b_in;
                    min_pos = i;
                }
            }
            b[min_pos] += 1;
            ans.push(min_pos + 1);
        }
    }
    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
