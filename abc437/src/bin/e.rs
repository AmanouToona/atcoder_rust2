use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn dfs(u: usize, is: &[Vec<usize>], to: &[BTreeMap<usize, usize>], ans: &mut Vec<usize>) {
    ans.extend_from_slice(&is[u]);

    for (_, v) in to[u].iter() {
        dfs(*v, is, to, ans);
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        xy: [(usize, usize); N]
    }

    let mut is: Vec<Vec<usize>> = vec![vec![0]]; // node に属する a の番号
    let mut to: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new()]; // node の持つ edge
    let mut vid: Vec<usize> = vec![0; N + 1]; // a -> node

    for (i, &(x, y)) in xy.iter().enumerate() {
        let parent = vid[x]; // 親ノードの番号
        let i = i + 1;

        if let Some(nxt) = to[parent].get(&y) {
            vid[i] = *nxt;
            is[*nxt].push(i);
        } else {
            let nxt_nid = is.len();
            is.push(vec![i]);
            to.push(BTreeMap::new());

            to[parent].insert(y, nxt_nid);
            vid[i] = nxt_nid;
        };
    }

    let mut ans = Vec::new();
    dfs(0, &is, &to, &mut ans);

    let ans: String = ans.iter().skip(1).join(" ");
    println!("{ans}");
}
