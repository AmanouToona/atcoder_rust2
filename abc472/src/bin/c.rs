#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M, K): (usize, usize, usize),
        A: [usize; N],
    }

    let mut eat = vec![false; N];
    let mut stack_cal = 0;
    for (i, &a) in A.iter().enumerate() {
        if i >= M && eat[i - M] {
            stack_cal -= A[i - M];
        }

        if stack_cal + a <= K {
            stack_cal += a;
            eat[i] = true;
        }
    }

    for &i in eat.iter() {
        if i {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
