#![allow(non_snake_case)]
use amplify::confinement::Collection;
use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;
/*
1 -> N の最短経路に使われている辺以外をカットした場合は答えは変わらない。
最短経路上に存在数r辺を削除すると答えが変化する.
切ったあとは N を含む最短経路上に存在する頂点に戻る経路を辿る。
最短経路からの逆順で辿った結果を持っておけば良い？ 最短経路上の最も近い頂点から2つ分方保持？ いや、戻ったら意味がない
再探索が必要になるのは高々N回 一度の探索で O(N + M) かかるので全体でも O(N (N + M)) ~ O(N **3) で間に合う。
*/

// 特定の辺を利用せずにたどり着く方法
fn search(g: &[Vec<(usize, usize)>], not_use: usize) -> i64 {
    let mut q = VecDeque::new();
    let mut dist = vec![i64::MAX; g.len()];
    dist[0] = 0;
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        for &(v, edge_no) in g[u].iter() {
            if edge_no == not_use {
                continue;
            }
            if dist[v] <= dist[u] + 1 {
                continue;
            }
            dist[v] = dist[u] + 1;
            q.push_back(v);
        }
    }

    if *dist.last().unwrap() != i64::MAX {
        dist[g.len() - 1]
    } else {
        -1
    }
}
fn main() {
    input! {
        (N, M): (usize, usize),
        st: [(usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];
    for (i, &(s, t)) in st.iter().enumerate() {
        let s = s - 1;
        let t = t - 1;

        g[s].push((t, i));
    }

    // 最短経路と利用する辺を探索
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut dist = vec![usize::MAX; N];
    let mut edge_from = vec![0; N];
    dist[0] = 0;
    while let Some(u) = q.pop_front() {
        for &(v, e) in g[u].iter() {
            if dist[v] <= dist[u] + 1 {
                continue;
            }
            q.push_back(v);
            dist[v] = dist[u] + 1;
            edge_from[v] = e;
        }
    }

    if *dist.last().unwrap() == usize::MAX {
        for _ in 0..M {
            println!("-1");
        }
        return;
    }

    // 最短経路で利用する辺を保存
    let mut edge_use = HashSet::new();
    let mut u = N - 1;
    while u != 0 {
        edge_use.push(edge_from[u]);
        (u, _) = st[edge_from[u]];
        u -= 1
    }

    // 回答を作成
    for i in 0..M {
        if edge_use.contains(&i) {
            let ans = search(&g, i);
            println!("{ans}");
        } else {
            println!("{}", dist[N - 1]);
        }
    }
}
