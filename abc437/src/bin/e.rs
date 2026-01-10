use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn dfs(u: usize, g: &Vec<BTreeMap<usize, Vec<usize>>>, ans: &mut Vec<usize>) {
    let gu = g[u].clone();
    for v_s in gu.values() {
        for v in v_s.iter() {
            ans.push(*v);
        }
        for v in v_s.iter() {
            dfs(*v, g, ans);
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        xy: [(usize, usize); N]
    }

    let mut g: Vec<BTreeMap<usize, Vec<usize>>> = vec![BTreeMap::new(); N + 1];
    for (i, &(x, y)) in xy.iter().enumerate() {
        let i = i + 1;
        g[x].entry(y).or_default().push(i);
    }

    let mut ans = Vec::new();
    dfs(0, &g, &mut ans);
    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
