#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Chars;

fn lt_span(S: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let mut h_min = usize::MAX;
    let mut h_max = 0;
    let mut w_min = usize::MAX;
    let mut w_max = 0;

    for h in 0..S.len() {
        for (w, &s) in S[h].iter().enumerate() {
            if s == '.' {
                continue;
            }

            h_min = h_min.min(h);
            h_max = h_max.max(h);
            w_min = w_min.min(w);
            w_max = w_max.max(w);
        }
    }

    if h_min == usize::MAX {
        (0, 0, 0, 0)
    } else {
        (h_min, w_min, h_max - h_min + 1, w_max - w_min + 1)
    }
}

fn main() {
    input! {
        N: usize,
        S: [Chars; N],
        T: [Chars; N],
    }

    let mut pre = S.clone();

    for _ in 0..4 {
        let mut nxt = S.clone();

        for h in 0..N {
            for w in 0..N {
                nxt[w][N - h - 1] = pre[h][w]
            }
        }

        let (t_t, t_l, t_h, t_w) = lt_span(&T);
        let (s_t, s_l, s_h, s_w) = lt_span(&nxt);

        if t_h == s_h && t_w == s_w {
            let mut ans = true;
            'judge: for h in 0..t_h {
                for w in 0..t_w {
                    if T[t_t + h][t_l + w] != nxt[s_t + h][s_l + w] {
                        ans = false;
                        break 'judge;
                    }
                }
            }
            if ans {
                println!("Yes");
                return;
            }
        }
        pre = nxt;
    }
    println!("No");
}
