#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

/*
- 木DP

dis(i, j) を愚直に求めると O(NN)
N = 2*10**5 なので、無理

dp[u][p] := 頂点 u 親 p としたときに葉までの距離の相和
up をサイズN でとると無理だが、 u-p の組み合わせは辺の数しか存在しない。 つまり高々 N - 1 の組み合わせしかない. 逆にたどっても 2 (N - 1)

dp[u][p] = sum (dp[v][u] + vを含めて v の先にある頂点数)
v は u の子
頂点数により、加算する数が異なる ...
dp に、距離の総和だけでなく頂点すうも持たせる

葉では?
dp[u][p] = sum dp[v][u] で、 v が存在しないので 0

最初の点は?
u をループして親は自分自身とすれば良い

*/

fn dfs(u: usize, p: usize, g: &[Vec<usize>], dp: &mut [HashMap<usize, (usize, usize)>]) {
    let mut sum_edge = 0;
    let mut sum_node = 1;
    for &v in g[u].iter() {
        if v == p {
            continue;
        }

        if !dp[v].contains_key(&u) {
            dfs(v, u, g, dp);
        }

        sum_edge += dp[v].get(&u).unwrap().0;
        sum_node += dp[v].get(&u).unwrap().1;
    }

    dp[u].insert(p, (sum_edge + sum_node - 1, sum_node));
}

fn main() {
    input! {
        N: usize,
        uv: [(usize, usize); N - 1],
    }

    let mut g = vec![Vec::new(); N];
    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push(v);
        g[v].push(u);
    }

    // dp[u][v] := (親をvとしたときにuから先にたどり着くための合計の距離, vを含めvから先にある頂点の数)
    let mut dp: Vec<HashMap<usize, (usize, usize)>> = vec![HashMap::new(); N];
    for u in 0..N {
        dp[u].insert(u, (0, 1));
    }

    for u in 0..N {
        dfs(u, u, &g, &mut dp);
    }

    for (i, h) in dp.iter().enumerate() {
        let ans = h.get(&i).unwrap().0;
        println!("{ans}");
    }
}
