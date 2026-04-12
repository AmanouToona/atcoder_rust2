#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn dfs(n: usize, m: usize, val: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>) {
    if val.len() == n {
        ans.push(val.clone());
        return;
    }

    let mut nxt = val.last().unwrap() + 10;
    while nxt <= m {
        val.push(nxt);
        dfs(n, m, val, ans);
        val.pop();
        nxt += 1;
    }
}
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut ans = Vec::new();
    for i in 1..=M {
        let mut val = Vec::new();
        val.push(i);
        dfs(N, M, &mut val, &mut ans);
    }

    println!("{}", ans.len());
    for i in ans.iter() {
        let ans = i.iter().join(" ");
        println!("{ans}");
    }
}
