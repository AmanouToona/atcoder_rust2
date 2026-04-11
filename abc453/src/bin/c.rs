#![allow(non_snake_case)]
use proconio::input;

fn dfs(cnt: usize, p: f64, i: usize, L: &Vec<f64>) -> usize {
    if i >= L.len() {
        return cnt;
    }

    let mut res = cnt;
    for d in [1f64, -1f64] {
        let nxt_p = p + d * L[i];
        let nxt_cnt = if p * nxt_p < 0. { cnt + 1 } else { cnt };
        res = res.max(dfs(nxt_cnt, nxt_p, i + 1, L));
    }

    res
}

fn main() {
    input! {
        N: usize,
        L: [f64; N],
    }

    let ans = dfs(0, 0.5, 0, &L);
    println!("{ans}")
}
