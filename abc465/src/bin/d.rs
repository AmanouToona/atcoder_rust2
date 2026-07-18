#![allow(non_snake_case)]
use proconio::input;

/*
- あまりに着目するわけでもない
- x, y が大きすぎる。　そうでなければ dp でいける
- 実は小問題に落とせるのでは？ ... ユークリッドの互除法的な

*/
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {(X, Y, K): (usize, usize, usize)}

        let mut ans = 0;
        let mut x = X;
        let mut y = Y;
        while x != y {
            if x > y {
                x /= K;
            } else {
                y /= K;
            }
            ans += 1;
        }
        println!("{ans}");
    }
}
