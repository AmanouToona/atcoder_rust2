#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        N: usize,
    }

    let mut ans = vec![Vec::new(); N];
    for i in 0..N {
        input! {
            K: usize,
            A: [usize; K],
        }

        for &a in A.iter() {
            ans[a - 1].push(i);
        }
    }

    for (i, an) in ans.iter().enumerate() {
        let ans_string: String = an.iter().map(|x| x + 1).join(" ");
        println!("{} {ans_string}", an.len())
    }
}
