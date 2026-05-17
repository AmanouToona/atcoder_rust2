#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        mut K: usize
    }

    let mut A = Vec::new();
    for _ in 0..N {
        input! {L: usize, a: [usize; L]}
        A.push(a);
    }
    input! {C: [usize; N]}

    K -= 1;
    for (i, &c) in C.iter().enumerate() {
        if K >= A[i].len() * c {
            K -= A[i].len() * c;
        } else {
            K %= A[i].len();
            println!("{}", A[i][K]);
            return;
        }
    }
}
