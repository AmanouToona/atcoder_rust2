#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (H, W): (usize, usize),
        S: [Chars; H],
    }

    let mut ans = 0;
    for h1 in 0..H {
        for h2 in h1..H {
            for w1 in 0..W {
                for w2 in w1..W {
                    let mut is_ans = true;
                    for i in h1..=h2 {
                        for j in w1..=w2 {
                            let h = h1 + h2 - i;
                            let w = w1 + w2 - j;

                            if h >= H || w >= W {
                                continue;
                            }

                            if S[h][w] != S[i][j] {
                                is_ans = false;
                            }
                        }
                    }

                    if is_ans {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{ans}");
}
