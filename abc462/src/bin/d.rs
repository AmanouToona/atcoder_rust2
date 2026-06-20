#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
fn main() {
    input! {
        (N, D): (usize, usize),
        mut st: [(usize, usize); N],
    }

    // st.sort_by_key(|x| x.0);
    let time_max = 3_000_000;
    let mut room_out_time = vec![Vec::new(); time_max];
    for &(s, t) in st.iter() {
        room_out_time[s].push(t);
    }

    let mut ans = 0;

    let mut room = BinaryHeap::new();
    for time in 0..=1_000_000 {
        for &out in room_out_time[time].iter() {
            room.push(Reverse(out));
        }

        while room.peek().is_some_and(|Reverse(pre)| *pre < time + D) {
            room.pop();
        }

        let len = room.len();
        // eprintln!("{:?}", room);

        if len >= 2 {
            ans += len * (len - 1) / 2;
        }
    }

    println!("{ans}");
}
