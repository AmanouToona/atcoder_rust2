#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/*
L..=R に R - L + 1 より多くの数字があったらだめ。
セグ木でできそうだが、 L, R が 10**9 なので、そのままでは無理。

テストケースが 10**5 あるので、O(N) の計算量だと嬉しい.

座標圧縮して左から貪欲に決めていく？
Lに複数個の弾があった場合の処理が難しい.

L + 1..=R までに含んで大丈夫な球の数を持ち続ける？
L が重なっても大丈夫 許容量を - していけばいい
次の L が今の R を超えるなら、今までの制約はクリアしたと考えられる。
そうでないならば、Rの増分だけ許容量を増やす

この貪欲の保証...ないな

でも、弾が入っている領域を左に寄せていく発想は良さそうに思う

---
入れられる玉で一番Rが小さいものを選択して入れていく

*/

fn main() {
    input! {
        T: usize,
    }

    'outer: for _ in 0..T {
        input! {
            N: usize,
            mut lr: [(usize, usize); N],
        }

        lr.sort_by_key(|x| x.0);

        let mut bx = 0; // box
        let mut iter = lr.iter().peekable();
        let mut q = BinaryHeap::new();
        while iter.peek().is_some() || !q.is_empty() {
            if q.is_empty() {
                if let Some(&(l, _)) = iter.peek() {
                    bx = bx.max(*l);
                }
            }

            while let Some(&(_, r)) = iter.next_if(|(l, _)| *l <= bx) {
                q.push(Reverse(r));
            }

            let Some(Reverse(min)) = q.pop() else {
                break;
            };

            if min < bx {
                println!("No");
                continue 'outer;
            }
            bx += 1;
        }

        println!("Yes");
    }
}
