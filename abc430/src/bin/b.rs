use itertools::iproduct;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        S: [Chars; N],
    }

    let mut set = HashSet::new();

    for (i, j) in iproduct!((0..=N - M), (0..=N - M)) {
        let mut s = Vec::new();

        for (di, dj) in iproduct!((0..M), (0..M)) {
            s.push(S[i + di][j + dj]);
        }
        set.insert(s.iter().collect::<String>());
    }

    println!("{}", set.len());
}
