#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
/*
ソートの処理が難しい
ソートされるまでは deque で順番に処理できる

ソート後の状態を heap でもつ
ソート前の状態を deque でもつ
すでに消費しているかを vec でもつ

q2
heap が存在する限りは heap から取り出す
heap がないならば deque から取り出す

q3
deque の内容を取り出して heap に追加する

これで、消費済みであるかを管理する必要はなくなる

*/

fn main() {
    input! {
        Q: usize
    }

    let mut heap = BinaryHeap::new();
    let mut que: VecDeque<usize> = VecDeque::new();
    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                input! {x:usize}
                que.push_back(x);
            }
            2 => {
                if heap.is_empty() {
                    if let Some(ans) = que.pop_front() {
                        println!("{ans}");
                    }
                } else if let Some(Reverse(ans)) = heap.pop() {
                    println!("{ans}");
                }
            }
            3 => {
                while let Some(q) = que.pop_front() {
                    heap.push(Reverse(q));
                }
            }
            _ => {
                panic!()
            }
        }
    }
}
