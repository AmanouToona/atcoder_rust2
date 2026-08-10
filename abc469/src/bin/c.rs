#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::{eprintln, println};

/*
意味はわかる 難しい感じはしないが、計算量を落とせない
処理するべき情報は何か?

- 後いくつ右に動いて良いか?  が各マスについて得られる
- 自分の左のます or 右のマスの情報を利用できないだろうか?
    - 右のますの情報を利用するのは無理だろう
    - kanade's algo らしさを感じるが少し違うか
- 愚直なら O(NN) だから、前計算の情報を使いたい

*/

fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let mut cumsum = vec![0; N + 1];
    for (i, &s) in S.iter().enumerate() {
        if s == 'o' {
            cumsum[i + 1] += 1;
        }
        cumsum[i + 1] += cumsum[i];
    }

    let mut ans = vec![0; N];

    for i in 0..N {
        let mut pre = i + 1;
        let mut leeway = cumsum[pre];
        let mut nxt = (pre + leeway).min(N);

        while nxt != pre {
            leeway = cumsum[nxt] - cumsum[pre];
            pre = nxt;
            nxt = (nxt + leeway).min(N);
        }

        ans[i] = nxt;
    }

    for a in ans.iter() {
        println!("{a}");
    }
}
