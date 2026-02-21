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
        if map.contains_key(&(a - 1)) {
            let now = *map.entry(a).or_insert(1);
            let new = now.max(*map.get(&(a - 1)).unwrap() + 1);
            *map.entry(a).or_default() = new;
        } else {
            let now = *map.entry(a).or_insert(1);
        }
    }
    let mut ans = 0;
    for (&k, &v) in map.iter() {
        ans = ans.max(v);
    }

    println!("{ans}");
}
