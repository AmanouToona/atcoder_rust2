#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

/*
# 逆に同じになる文字列の個数を数える？
同じになる文字列であれば、尺取りっぽくできる... ? いやできない
CCABAB とかの時、Cが多いからといって、左を進めるとダメになる

- それぞれ管理する？

2文字だったらどする？
AAAABBBB などをうまく動かす方法はあるか？

* 0　回も重複カウント ... "A" は、B, C が0回だから out

- 差分の管理？
C - A = - (B - C) - (A - B) なので
B - C, A - B の個数の2つの状態だけ管理
もしくは、 A に対して x個多いという情報を管理？ これも2つ

*/

fn main() {
    input! {
        N: usize,
        S: Chars,
    }
}
