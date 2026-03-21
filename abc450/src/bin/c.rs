#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
fn main() {
    input! {
        (H, W): (usize, usize),
        mut S: [Chars; H],
    }

    let d = [(0, 1), (!0, 0), (0, !0), (1, 0)];

    // 外周部に隣接する白を着色
    for h in 0..H {
        for w in 0..W {
            if (h != 0 && h != H - 1) && (w != 0 && w != W - 1) {
                continue;
            }
            if S[h][w] == '#' {
                continue;
            }

            let mut q = VecDeque::new();
            q.push_back((h, w));

            while let Some((uh, uw)) = q.pop_front() {
                if S[uh][uw] == '#' {
                    continue;
                }
                S[uh][uw] = '#';

                for &(dh, dw) in d.iter() {
                    let vh = uh.wrapping_add(dh);
                    let vw = uw.wrapping_add(dw);
                    if vh >= H || vw >= W {
                        continue;
                    }
                    if S[vh][vw] == '#' {
                        continue;
                    }
                    q.push_back((vh, vw));
                }
            }
        }
    }

    // for s in S.iter() {
    //     eprintln!("{:?}", s);
    // }

    // 回答をカウント
    let mut ans = 0;
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '#' {
                continue;
            }

            ans += 1;
            let mut q = VecDeque::new();
            q.push_back((h, w));

            while let Some((uh, uw)) = q.pop_front() {
                if S[uh][uw] == '#' {
                    continue;
                }
                S[uh][uw] = '#';

                for &(dh, dw) in d.iter() {
                    let vh = uh.wrapping_add(dh);
                    let vw = uw.wrapping_add(dw);
                    if vh >= H || vw >= W {
                        continue;
                    }
                    if S[vh][vw] == '#' {
                        continue;
                    }
                    q.push_back((vh, vw));
                }
            }
        }
    }
    //     for s in S.iter() {
    //     eprintln!("{:?}", s);
    // }
    println!("{ans}");

}
