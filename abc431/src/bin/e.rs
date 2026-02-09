use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {T: usize}

    // 上から時計回りに入って来た方向を保存する
    // dは整合性を持って設定
    let d: [(usize, usize); 4] = [(1, 0), (0, !0), (!0, 0), (0, 1)];
    for _ in 0..T {
        input! {
            (H, W): (usize, usize),
            S: [Chars; H],
        }

        // h, w, direction, cost,
        let mut q: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
        let mut distance = vec![vec![vec![usize::MAX; 4]; W]; H];
        q.push_front((0, 0, 3, 0));
        distance[0][0][3] = 0;

        let mut ans = usize::MAX;
        while let Some((uh, uw, udir, ucost)) = q.pop_front() {
            if ucost > distance[uh][uw][udir] {
                continue;
            }

            // 0 コストでいける方向を dir0 とする
            let mut dir0 = udir;
            if S[uh][uw] == 'B' {
                dir0 ^= 3;
            } else if S[uh][uw] == 'C' {
                dir0 ^= 1;
            };

            for (i, &(dh, dw)) in d.iter().enumerate() {
                let vcost = ucost + if i == dir0 { 0 } else { 1 };
                let vh = uh.wrapping_add(dh);
                let vw = uw.wrapping_add(dw);

                if vh == H - 1 && vw == W {
                    ans = ans.min(vcost);
                }

                if vh >= H || vw >= W {
                    continue;
                }

                if distance[vh][vw][i] <= vcost {
                    continue;
                }

                distance[vh][vw][i] = vcost;

                if i == dir0 {
                    q.push_front((vh, vw, i, vcost));
                } else {
                    q.push_back((vh, vw, i, vcost));
                }
            }
        }
        println!("{ans}");
    }
}
