#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
fn main() {
    input! {
        (H, W): (usize, usize),
        S: [Chars; H]
    }

    let mut sh = 0;
    let mut sw = 0;
    let mut gh = 0;
    let mut gw = 0;
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == 'S' {
                sh = h;
                sw = w;
            } else if S[h][w] == 'G' {
                gh = h;
                gw = w;
            }
        }
    }

    let d = [(0, 1, 'D'), (!0, 0, 'L'), (0, !0, 'U'), (1, 0, 'R')];

    let mut dp: Vec<Vec<Vec<Option<(char, usize, usize, usize)>>>> =
        vec![vec![vec![None; 4]; W]; H];
    let mut q = VecDeque::new();
    for (i, &d) in d.iter().enumerate() {
        dp[sh][sw][i] = Some(('S', 0, 0, 0));
        q.push_back((sh, sw, i));
    }

    'outer: while let Some((uh, uw, direction)) = q.pop_front() {
        for (i, &(dw, dh, dc)) in d.iter().enumerate() {
            let vh = uh.wrapping_add(dh);
            let vw = uw.wrapping_add(dw);

            if vh >= H || vw >= W || S[vh][vw] == '#' || dp[vh][vw][i].is_some() {
                continue;
            }

            if S[uh][uw] == 'o' && i != direction {
                continue;
            }

            if S[uh][uw] == 'x' && i == direction {
                continue;
            }

            dp[vh][vw][i] = Some((dc, uh, uw, direction));

            if vh == gh && vw == gw {
                break 'outer;
            }
            q.push_back((vh, vw, i));
        }
    }

    // 復元
    let mut ans = Vec::new();

    if dp[gh][gw].iter().all(|x| x.is_none()) {
        println!("No");
        return;
    }

    let mut h = gh;
    let mut w = gw;
    let mut i = 0;
    for d in 0..4 {
        if dp[gh][gw][d].is_some() {
            i = d;
            break;
        }
    }

    // for i in dp.iter() {
    //     eprintln!("{:?}", i);
    // }
    loop {
        if h == sh && w == sw {
            break;
        }
        // eprintln!("{h} {w} {i}");
        // eprintln!("{:?}", dp[h][w]);
        // eprintln!("{:?}", dp[h][w][i]);
        let uh = h;
        let uw = w;
        let ui = i;
        ans.push(dp[h][w][i].unwrap().0);
        h = dp[uh][uw][ui].unwrap().1;
        w = dp[uh][uw][ui].unwrap().2;
        i = dp[uh][uw][ui].unwrap().3;
    }

    let ans: String = ans.iter().rev().join("");
    println!("Yes");
    println!("{ans}");
}
