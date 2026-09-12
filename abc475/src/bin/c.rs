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

    let mut ans: usize = 1;
    for left in 0..=S {
        let left_dist: usize = dist[left].abs_diff(dist[S]);
        if left_dist > L {
            continue;
        }

        let mut ll: usize = S;
        let mut right: usize = N;
        while right - ll > 1 {
            let mid: usize = (right + ll) / 2;
            if left_dist * 2 + dist[mid] - dist[S] > L {
                right = mid;
            } else {
                ll = mid;
            }
        }
        ans = ans.max(ll - left + 1);
    }

    for right in S..N {
        let right_dist: usize = dist[right] - dist[S];
        if right_dist > L {
            continue;
        }

        if right_dist * 2 + dist[S] - dist[0] <= L {
            ans = ans.max(right - 0 + 1);
        }

        let mut ll: usize = 0;
        let mut rr: usize = S;
        while rr - ll > 1 {
            let mid: usize = (rr + ll) / 2;

            if right_dist * 2 + dist[S] - dist[mid] > L {
                ll = mid;
            } else {
                rr = mid;
            }
        }
        ans = ans.max(right - ll);
    }

    println!("{ans}");
}
