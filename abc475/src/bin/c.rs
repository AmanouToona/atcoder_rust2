#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, S, L): (usize, usize, usize),
        A: [usize; N - 1],
    }

    let S: usize = S - 1;
    let mut dist = vec![0; N];
    for (i, &a) in A.iter().enumerate() {
        dist[i + 1] = a;
        dist[i + 1] += dist[i];
    }

    let mut ans = 1;
    for l in 0..N {
        for r in l..N {
            if l > S || r < S {
                continue;
            }

            if (dist[S] - dist[l]) * 2 + (dist[r] - dist[S]) <= L
                || (dist[S] - dist[l]) + (dist[r] - dist[S]) * 2 <= L
            {
                ans = ans.max(r - l + 1);
            }
        }
    }

    println!("{ans}");
}
