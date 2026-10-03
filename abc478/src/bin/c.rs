#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }

    let mut goal = A.clone();
    goal.sort();

    if A == goal {
        println!("Yes");
        return;
    }

    let left = A.iter().zip(goal.iter()).position(|x| x.0 != x.1).unwrap();
    let right = A.iter().zip(goal.iter()).rposition(|x| x.0 != x.1).unwrap();

    if right - left < K {
        println!("Yes");
    } else {
        println!("No");
    }
}
