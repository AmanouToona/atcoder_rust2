use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        S: Chars,
    }

    let mut cnt = HashMap::new();

    for i in 0..N - K + 1 {
        *cnt.entry(S.iter().skip(i).take(K).cloned().collect::<String>())
            .or_default() += 1;
    }

    let max: usize = cnt.values().cloned().max().unwrap();

    let mut ans = cnt
        .iter()
        .filter(|&(_, cnt)| *cnt == max)
        .map(|x| x.0.clone())
        .collect::<Vec<String>>();

    ans.sort();

    println!("{}", max);
    println!("{}", ans.iter().join(" "));
}
