#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        (H, W): (usize, usize),
        S: [Chars; H],
        (A, B, C, D): (usize, usize, usize, usize),
    }

    let sh = A - 1;
    let sw = B - 1;
    let gh = C - 1;
    let gw = D - 1;

    let d = [(0, 1), (!0, 0), (0, !0), (1, 0)];
    let mut map = vec![vec![1 << 60; W]; H];
    let mut q: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();

    q.push((Reverse(0), sh, sw));
    map[sh][sw] = 0;

    while let Some((Reverse(cost), uh, uw)) = q.pop() {
        if map[uh][uw] < cost {
            continue;
        }

        for &(dh, dw) in d.iter() {
            let vh = uh.wrapping_add(dh);
            let vw = uw.wrapping_add(dw);

            if vh >= H || vw >= W {
                continue;
            }

            if S[vh][vw] == '.' && map[vh][vw] > cost {
                q.push((Reverse(cost), vh, vw));
                map[vh][vw] = cost;
            } else if S[vh][vw] == '#' {
                let v_cost = cost + 1;
                if map[vh][vw] > v_cost {
                    q.push((Reverse(v_cost), vh, vw));
                    map[vh][vw] = v_cost;
                }

                let vvh = vh.wrapping_add(dh);
                let vvw = vw.wrapping_add(dw);
                if vvh < H && vvw < W && map[vvh][vvw] > v_cost {
                    q.push((Reverse(v_cost), vvh, vvw));
                    map[vvh][vvw] = v_cost;
                }
            }
        }
    }

    println!("{}", map[gh][gw]);
}
