#![allow(non_snake_case)]
use amplify::confinement::Collection;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
use std::collections::VecDeque;
fn main() {
    input! {
        (H, W, K): (usize, usize, usize),
        S: [Chars; H],
    }

    let mut count = vec![vec![0; W]; H];
    let mut danger_h = HashSet::new();
    let mut danger_w = HashSet::new();
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '#' {
                danger_h.insert(h);
                danger_w.insert(w);
            }
        }
    }

    for &h in danger_h.iter() {
        for w in 0..W {
            count[h][w] = usize::MAX;
        }
    }

    for &w in danger_w.iter() {
        for h in 0..H {
            count[h][w] = usize::MAX;
        }
    }

    let mut q = VecDeque::new();
    for h in 0..H {
        for w in 0..W {
            if count[h][w] == 0 {
                q.push((h, w));
            }
        }
    }

    let mut d = [(1, 0), (0, !0), (!0, 0), (0, 1)];
    while let Some((uh, uw)) = q.pop_front() {
        let nxt_count = count[uh][uw] + 1;
        if nxt_count > K {
            continue;
        }

        for &(dh, dw) in d.iter() {
            let nxt_h = uh.wrapping_add(dh);
            let nxt_w = uw.wrapping_add(dw);

            if nxt_h >= H || nxt_w >= W {
                continue;
            }
            if S[nxt_h][nxt_w] == '#' {
                continue;
            }
            if count[nxt_h][nxt_w] <= nxt_count {
                continue;
            }
            count[nxt_h][nxt_w] = nxt_count;
            q.push_back((nxt_h, nxt_w));
        }
    }

    let mut ans = 0;
    for h in 0..H {
        for w in 0..W {
            if count[h][w] <= K {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
