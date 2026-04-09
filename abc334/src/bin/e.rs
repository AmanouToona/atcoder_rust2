#![allow(non_snake_case)]
use ac_library::Dsu;
use ac_library::ModInt998244353 as mint;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (H, W): (usize, usize),
        S: [Chars; H],
    }

    let mut dsu = Dsu::new(H * W);
    let mut count_red = mint::new(0);
    let d = [(1, 0), (0, !0), (!0, 0), (0, 1)];
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '.' {
                count_red += mint::new(1);
                continue;
            }
            let p = h * W + w;
            for &(dh, dw) in d.iter() {
                let vh = h.wrapping_add(dh);
                let vw = w.wrapping_add(dw);
                if vh < H && vw < W && S[vh][vw] == '#' {
                    let vp = vh * W + vw;
                    dsu.merge(p, vp);
                }
            }
        }
    }

    if count_red == mint::new(0) {
        println!("0");
        return;
    }

    let mut leader = std::collections::HashSet::new();
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '#' {
                let p = h * W + w;
                leader.insert(dsu.leader(p));
            }
        }
    }

    let n_group = leader.len();

    let mut ans = mint::new(0);
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '#' {
                continue;
            }

            let mut connect = std::collections::HashSet::new();
            for &(dh, dw) in d.iter() {
                let vh = h.wrapping_add(dh);
                let vw = w.wrapping_add(dw);
                if vh < H && vw < W && S[vh][vw] == '#' {
                    let vp = vh * W + vw;
                    connect.insert(dsu.leader(vp));
                }
            }

            ans += mint::new(n_group + 1 - connect.len());
        }
    }
    ans /= count_red;
    println!("{ans}");
}
