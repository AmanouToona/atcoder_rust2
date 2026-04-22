#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        S: [usize; N],
        T: [usize; N],
    }

    let mut time = T.clone();
    for _ in 0..2 {
        for (i, &s) in S.iter().enumerate() {
            time[(i + 1) % N] = time[(i + 1) % N].min(time[i] + s);
        }
    }

    for i in time.iter() {
        println!("{i}");
    }
}
