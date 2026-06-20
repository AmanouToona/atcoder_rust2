#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
/*
可能な限り小さなLに入れる
- L, Rが小さなボールから処理する
- L が小さいものは可能な限り小さいところに入れる
- L で走査
- 同じLではRが小さい方から入れる

実装
- L sort
- i = 0 -> ボールがないなら next L へ skip
- ボールがあるなら i+= 1, ball pop
- ball pop の結果(rを取得) i より大きいなら game over
*/

fn main() {
    input! {T: usize}
    'outer: for _ in 0..T {
        input! {
            N:usize,
            LR: [(usize, usize); N]
        }

        let mut ball_r: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        let mut box_cnt = 0;
        let mut lr = LR.clone();
        lr.sort_by_key(|x| x.0);

        let mut lr_iter = lr.iter().peekable();

        while box_cnt <= 1_000_000_000 {
            while let Some((l, r)) = lr_iter.peek() {
                if *l <= box_cnt {
                    lr_iter.next();
                    ball_r.push(Reverse(*r));
                } else {
                    break;
                }
            }

            if let Some(Reverse(r)) = ball_r.pop() {
                if r < box_cnt {
                    println!("No");
                    continue 'outer;
                }
                box_cnt += 1;
            }

            if ball_r.is_empty() {
                if let Some((l, _)) = lr_iter.peek() {
                    box_cnt = *l;
                } else {
                    break;
                }
            }
        }
        if ball_r.is_empty() {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
