#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
/*
平均値の最大値を求める問題
k <= (P[r + 1] - P[l]) / (r + 1 - l)
0 <= P[r + 1] - P[l] - k(r + 1) + kl
Q[i] = P[i] - k　とすると
0 <= Q[r + 1] - Q[l] をみたす l, r があるか？　という問題である
*/

fn judge(s: &[char], x: f64, k: usize) -> bool {
    let mut pref_sum = vec![0.];
    let mut sum_o = vec![0];

    for (i, &c) in s.iter().enumerate() {
        if c == 'x' {
            pref_sum.push(-x);
            sum_o.push(0);
        } else {
            pref_sum.push(1. - x);
            sum_o.push(1);
        }

        pref_sum[i + 1] += pref_sum[i];
        sum_o[i + 1] += sum_o[i];
    }

    let mut l = 0;
    let mut min_pref = f64::MAX;
    for r in 0..pref_sum.len() {
        while sum_o[r] - sum_o[l] >= k {
            min_pref = min_pref.min(pref_sum[l]);
            l += 1;
        }

        if min_pref <= pref_sum[r] {
            return true;
        }
    }

    false
}

fn main() {
    input! {
        (_, K): (usize, usize),
        S: Chars,
    }

    let mut low = 0.;
    let mut high = 1.;

    for _ in 0..30 {
        let mid = (high + low) / 2.;
        if judge(&S, mid, K) {
            low = mid;
        } else {
            high = mid;
        }
    }

    println!("{low}");
}
