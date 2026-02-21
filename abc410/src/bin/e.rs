#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, H, M): (usize, usize, usize),
        ab: [(usize, usize); N],
    }

    let mut max_m = vec![vec![None; 3001]; N + 1];
    max_m[0][H] = Some(M);

    for (i, &(a, b)) in ab.iter().enumerate() {
        for h in 0..=3000 {
            if let Some(m) = max_m[i][h] {
                if m >= b {
                    max_m[i + 1][h] = max_m[i + 1][h].max(Some(m - b));
                }
            }
            if h >= a {
                max_m[i + 1][h - a] = max_m[i + 1][h - a].max(max_m[i][h]);
            }
        }
    }

    let mut ans = 0;
    for i in 1..=N {
        for j in 0..=3000 {
            if max_m[i][j].is_some() {
                ans = i;
            }
        }
    }
    println!("{ans}");
}
