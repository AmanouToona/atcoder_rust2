#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();

    for &a in A.iter() {
        let &pre = map.get(&(a - 1)).unwrap_or(&0);
        map.entry(a)
            .and_modify(|x| *x = (*x).max(pre + 1))
            .or_insert(pre + 1);
    }

    let ans = map.values().fold(0, |acc, x| acc.max(*x));
    println!("{ans}");
}
