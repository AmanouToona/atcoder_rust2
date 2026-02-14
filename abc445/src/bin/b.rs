use itertools::{Itertools, repeat_n};
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
        let mut ans = vec!['.'; m];
        let start = (m - s.len()) / 2;
        for (i, ss) in s.iter().enumerate() {
            ans[i + start] = *ss;
        }
        let ans: String = ans.iter().join("");
        println!("{ans}")
    }
}
