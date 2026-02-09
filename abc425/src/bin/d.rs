#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
fn main() {
    input! {
        (H, W): (usize, usize),
        mut S: [Chars; H],
    }

    let mut q = VecDeque::new();
    let mut B = vec![vec![0; W]; H];

    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' {
                q.push_back((i, j));
                B[i][j] = 1;
            }
        }
    }

    let d = [(0, 1), (!0, 0), (0, !0), (1, 0)];
    while let Some((i, j)) = q.pop_front() {
        let next_num = B[i][j] + 1;
        for &(di, dj) in d.iter() {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);

            if ni >= H || nj >= W {
                continue;
            }

            if B[ni][nj] != 0 {
                continue;
            }

            let mut count_black = 0;
            for &(di, dj) in d.iter() {
                let check_i = ni.wrapping_add(di);
                let check_j = nj.wrapping_add(dj);
                if check_i >= H || check_j >= W {
                    continue;
                }

                if B[check_i][check_j] > 0 && B[check_i][check_j] < next_num {
                    count_black += 1;
                }
            }

            if count_black == 1 {
                B[ni][nj] = next_num;
                q.push_back((ni, nj));
            }
        }
    }

    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if B[i][j] != 0 {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
