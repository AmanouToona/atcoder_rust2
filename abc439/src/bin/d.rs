#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;

fn cnt(A: &[usize]) -> usize {
    let mut res = 0;
    let mut app: HashMap<usize, usize> = HashMap::new();

    for &a in A.iter() {
        if a % 5 == 0 {
            let i = a / 5;
            res += app.get(&(i * 7)).unwrap_or(&0) * app.get(&(i * 3)).unwrap_or(&0);
        }
        *app.entry(a).or_default() += 1;
    }

    res
}

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut ans = cnt(&A);
    let A = A.into_iter().rev().collect::<Vec<usize>>();
    ans += cnt(&A);

    println!("{ans}");
}
