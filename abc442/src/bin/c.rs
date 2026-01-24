#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (N, M): (usize, usize),
        AB: [(usize, usize); M],
    }

    let mut connection = vec![HashSet::new(); N];
    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;

        connection[a].insert(b);
        connection[b].insert(a);
    }

    let mut ans = Vec::new();
    for c in connection.iter() {
        let len = c.len();
        let can = N - len - 1;

        if can < 3 {
            ans.push(0);
            continue;
        }

        ans.push(can * (can - 1) * (can - 2) / 6);
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
