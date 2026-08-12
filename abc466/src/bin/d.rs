#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        RC: [(usize, usize); M],
    }

    let mut ignore_row = vec![false; N + 1];
    let mut ignore_col = vec![false; N + 1];
    let mut count = 0;
    for &(r, c) in RC.iter().rev() {
        if !ignore_col[c] && !ignore_row[r] {
            count += 1;
        }
        ignore_col[c] = true;
        ignore_row[r] = true;
    }

    println!("{count}");
}
