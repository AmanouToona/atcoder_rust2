#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut ing2dish = vec![Vec::new(); N + 1];
    let mut dishes = vec![HashSet::new(); M];
    for i in 0..M {
        input! {
            K: usize,
            A: [usize; K],
        }

        for &a in A.iter() {
            ing2dish[a].push(i);
            dishes[i].insert(a);
        }
    }

    input! {B: [usize; N]}

    let mut ans = 0;
    for &b in B.iter() {
        for &d in ing2dish[b].iter() {
            dishes[d].remove(&b);
            if dishes[d].is_empty() {
                ans += 1;
            }
        }
        println!("{ans}");
    }
}
