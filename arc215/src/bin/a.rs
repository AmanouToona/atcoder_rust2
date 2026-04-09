#![allow(non_snake_case)]
use proconio::input;
fn solve() {
    input! {
        (N, K, L): (usize, usize, usize),
        mut A: [usize; N],
    }

    A.sort();
    let mut diff: Vec<usize> = A.iter().zip(A.iter().skip(1)).map(|(x, y)| y - x).collect();
    diff.sort_by(|x, y| y.cmp(x));

    let dist1 = *A.iter().min().unwrap();
    let dist2 = L - A.iter().max().unwrap();
    let mut dist = dist1.max(dist2);
    let mut dist_tot = dist1 + dist2;

    let mut ans: usize = 0;
    let mut temp: usize = 0;

    for (i, remain) in (0..K).zip((1..=K).rev()) {
        if remain > 0 {
            ans = ans.max(temp + (remain - 1) * dist_tot + dist);
        }

        if i < diff.len() {
            temp += diff[i] / 2;
        }
        ans = ans.max(temp);

        if dist_tot < L {
            dist_tot += diff[i];
            dist += diff[i] / 2;
        } else {
            break;
        }
    }

    println!("{ans}");
}

fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        solve();
    }
}
