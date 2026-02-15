use itertools::{repeat_n, Itertools};
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let m = S.iter().map(|x| x.len()).max().unwrap();
    for s in S.iter() {
        let buff = (m - s.len()) / 2;
        let ans: String = repeat_n('.', buff)
            .chain(s.iter().cloned())
            .chain(repeat_n('.', buff))
            .join("");
        println!("{ans}")
    }
}
