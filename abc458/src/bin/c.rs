#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

/*
i 文字目の c を中心に考える (0index)
左端までの文字数は自身を含めて i + 1 個
右端目での文字数は自信を含めて len - i;
この c を中心とする文字列の個数は (i + i).min(len - i);

*/

fn main() {
    input! {
        S: Chars,
    }

    let mut ans = 0;
    for (i, &s) in S.iter().enumerate() {
        if s == 'C' {
            ans += (i + 1).min(S.len() - i);
        }
    }

    println!("{ans}");
}
