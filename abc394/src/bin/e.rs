#![allow(non_snake_case)]
use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
fn main() {
    input! {
        N: usize,
        C: [Chars; N],
    }

    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut ans = vec![vec![usize::MAX; N]; N];
    for i in 0..N {
        q.push((i, i, 0));
        ans[i][i] = 0;
    }

    // u to v
    for u in 0..N {
        for v in 0..N {
            if u != v && C[u][v] != '-' {
                ans[u][v] = 1;
                q.push((u, v, 1));
            }
        }
    }

    while let Some((u, v, c)) = q.pop_front() {
        if ans[u][v] < c {
            continue;
        }

        for uu in 0..N {
            if C[uu][u] != '-' {
                for vv in 0..N {
                    if C[uu][u] == C[v][vv] && ans[uu][vv] > c + 2 {
                        q.push_back((uu, vv, c + 2));
                        ans[uu][vv] = c + 2;
                    }
                }
            }
        }
    }

    for i in ans.iter() {
        let a: String = i
            .iter()
            .map(|&x| if x == usize::MAX { -1 } else { x as i64 })
            .join(" ");
        println!("{a}");
    }
}
