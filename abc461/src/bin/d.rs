#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

/*
マス目の数は最大で 2.5 * 10 ** 5
全探索は無理

書かれている数字が 1 であるということを使うのでは？
正の数しかないから 2分探索ができる？

HW 個の top left
buttom right もおおよそ HW 通りある

tl を固定した時、buttom right の H だけを振る W　は2分探索とする
計算量は O(H**2 W log(W)) ... 10 ** 8 くらい ギリギリ通る
*/

fn sum(dp: &[Vec<usize>], h1: usize, w1: usize, h2: usize, w2: usize) -> usize {
    let h1 = h1 + 1;
    let w1 = w1 + 1;
    let h2 = h2 + 1;
    let w2 = w2 + 1;

    dp[h2][w2] + dp[h1 - 1][w1 - 1] - dp[h1 - 1][w2] - dp[h2][w1 - 1]
}

fn main() {
    input! {
        (H, W, K): (usize, usize, usize),
        S: [Chars; H]
    }

    let mut dp = vec![vec![0; W + 1]; H + 1];
    for h in 0..H {
        for w in 0..W {
            dp[h + 1][w + 1] += if S[h][w] == '1' { 1 } else { 0 };
            dp[h + 1][w + 1] += dp[h][w + 1] + dp[h + 1][w] - dp[h][w];
        }
    }

    let mut ans = 0;
    for h in 0..H {
        for w in 0..W {
            // h, w が top left
            for h2 in h..H {
                // h2 が buttom

                // そもそも超えている
                if sum(&dp, h, w, h2, w) > K {
                    continue;
                }
                // そもそも足りない
                if sum(&dp, h, w, h2, W - 1) < K {
                    continue;
                }

                // K を超える方の探索
                let upper = if sum(&dp, h, w, h2, W - 1) == K {
                    W
                } else {
                    let mut left = w;
                    let mut right = W;
                    while right - left > 1 {
                        let mid = (right + left) / 2;
                        if sum(&dp, h, w, h2, mid) > K {
                            right = mid;
                        } else {
                            left = mid;
                        }
                    }
                    right
                };

                // K 未満となる方の探索
                let lower = if sum(&dp, h, w, h2, w) == K {
                    w - 1
                } else {
                    let mut left = w;
                    let mut right = W;
                    while right - left > 1 {
                        let mid = (right + left) / 2;
                        if sum(&dp, h, w, h2, mid) < K {
                            left = mid;
                        } else {
                            right = mid;
                        }
                    }
                    left
                };

                // この right より 1　小さいものは ok かも
                // eprintln!("{h} {w} {h2} {upper} {lower} {}", upper - lower - 1);
                ans += upper - 1 - lower;
            }
        }
    }
    println!("{ans}");
}
