#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {T: usize}

    for _ in 0..T {
        input! {
            N: usize,
            R: [usize; N],
        }

        let mut left = vec![R[0]];
        for (i, &r) in R.iter().enumerate().skip(1) {
            left.push(r.min(left[i - 1] + 1));
        }

        let mut right = vec![0; N];
        right[N - 1] = R[N - 1];
        for (i, &r) in R.iter().enumerate().rev().skip(1) {
            right[i] = r.min(right[i + 1] + 1);
        }

        let mut ans = 0;
        for (i, &r) in R.iter().enumerate() {
            ans += left[i].min(right[i]).abs_diff(r);
        }

        println!("{ans}");
    }
}
