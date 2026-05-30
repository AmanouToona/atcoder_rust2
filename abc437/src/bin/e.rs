#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;
/*
tri木を実装する問題?

必要な構造を整理する
- node
    - {次の状態}, {このノードの i}
- i -> node の逆引き

- vec![node]
- i -> node の vec
*/

fn dfs(u: usize, node: &Vec<BTreeMap<usize, usize>>, nci: &Vec<Vec<usize>>, ans: &mut Vec<usize>) {
    ans.extend_from_slice(&nci[u]);

    for &v in node[u].values() {
        dfs(v, node, nci, ans);
    }
}

fn main() {
    input! {
        N: usize,
        xy: [(usize, usize); N],
    }

    let mut node: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new()];
    let mut i2node: Vec<usize> = vec![0; N + 1];
    let mut node_contain_i: Vec<Vec<usize>> = vec![Vec::new()];

    for (i, &(x, y)) in xy.iter().enumerate() {
        let i = i + 1;

        if let Some(node) = node[i2node[x]].get(&y) {
            i2node[i] = *node;
            node_contain_i[*node].push(i);
        } else {
            let len = node.len();
            node[i2node[x]].insert(y, len);
            node.push(BTreeMap::new());
            node_contain_i.push(Vec::new());
            i2node[i] = node.len() - 1;
            node_contain_i[i2node[i]].push(i);
        }
    }

    let mut ans: Vec<usize> = Vec::with_capacity(N);

    dfs(0, &node, &node_contain_i, &mut ans);
    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
