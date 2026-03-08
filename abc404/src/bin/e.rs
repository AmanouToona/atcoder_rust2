#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        C: [usize; N - 1],
        A: [usize; N - 1],
    }

    let A: Vec<usize> = std::iter::once(0).chain(A).collect();
    let C: Vec<usize> = std::iter::once(0).chain(C).collect();

    let position: Vec<usize> = std::iter::once(0)
        .chain(
            A.iter()
                .enumerate()
                .filter(|&(_, a)| *a != 0)
                .map(|(i, _)| i),
        )
        .collect();

    let mut dp = vec![usize::MAX; N];
    dp[*position.last().unwrap()] = 0;
    for (&to, &from) in position.iter().zip(position.iter().skip(1)).rev() {
        for u in (to..=from).rev() {
            for d in 0..=C[u] {
                let v = u.wrapping_sub(d).max(to);
                dp[v] = dp[v].min(dp[u].wrapping_add(1));
            }
        }
    }

    println!("{}", dp[0]);
}
