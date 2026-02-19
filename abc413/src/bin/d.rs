#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {T: usize}
    'outer: for _ in 0..T {
        input! {
            N: usize,
            mut A: [i64; N],
        }

        // 公比 1, -1
        if A.iter().all(|x| *x == A[0]) {
            println!("Yes");
            continue 'outer;
        }

        let mut kinds = HashSet::new();
        let mut diff: i32 = 0;
        for &a in A.iter() {
            if a < 0 {
                diff -= 1;
                kinds.insert(a.abs());
            } else {
                kinds.insert(a);
                diff += 1;
            }
        }

        if kinds.len() == 1 && diff.abs() <= 1 {
            println!("Yes");
            continue 'outer;
        }

        A.sort_by_key(|x| x.abs());
        for i in 0..N - 2 {
            if A[i] * A[i + 2] != A[i + 1] * A[i + 1] {
                println!("No");
                continue 'outer;
            }
        }
        println!("Yes");
    }
}
