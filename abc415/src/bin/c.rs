#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main() {
    input! {T: usize}
    'outer: for _ in 0..T {
        input! {
            N: usize,
            S: Chars,
        }

        let mut states = HashSet::new();
        states.insert(0);
        let mut checked = vec![false; 1 << N];
        checked[0] = true;

        while !states.is_empty() {
            let mut nxt_states = HashSet::new();

            for &state in states.iter() {
                for i in 0..N {
                    if state >> i & 1 == 1 {
                        continue;
                    }
                    let nxt = state | (1 << i);
                    if S[nxt - 1] == '1' {
                        // danger!!
                        continue;
                    }
                    if nxt == (1 << N) - 1 {
                        println!("Yes");
                        continue 'outer;
                    }
                    if checked[nxt] {
                        continue;
                    }
                    nxt_states.insert(nxt);
                    checked[nxt] = true;
                }
            }
            states = nxt_states;
        }
        println!("No");
    }
}
