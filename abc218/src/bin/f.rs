#![allow(non_snake_case)]
use amplify::confinement::Collection;
use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;
/*
- 最短経路上にない変を削除しても答えは変化しない
- 最短経路上の変の数は高々 n
- 最短経路の算出は O(n + m)

- 最短経路上の辺を消した時のみ再計算すれば O(n(n + m)) ... O(n ** 3)
*/

fn bfs(cant_use: usize, g: &[Vec<(usize, usize)>]) -> Option<HashSet<usize>> {
    let n = g.len();
    let mut q = VecDeque::new();
    q.push_back(0);

    // [(parent node num, edge num)]
    let mut frm: Vec<Option<(usize, usize)>> = vec![None; n];

    while let Some(u) = q.pop_front() {
        for &(v, i) in g[u].iter() {
            if i == cant_use {
                continue;
            }
            if frm[v].is_some() {
                continue;
            }
            frm[v] = Some((u, i));
            q.push_back(v);
        }
    }

    // 到達不能
    frm[n - 1]?;

    // 再構築
    let mut used_edge = HashSet::new();
    let mut i = n - 1;
    while i != 0 {
        used_edge.push(frm[i].unwrap().1);
        i = frm[i].unwrap().0;
    }

    Some(used_edge)
}

fn main() {
    input! {
        (N, M): (usize, usize),
        st: [(usize, usize); M]
    }

    let mut g = vec![Vec::new(); N];
    for (i, &(s, t)) in st.iter().enumerate() {
        g[s - 1].push((t - 1, i));
    }

    // 最短経路の探索
    let shortest_travel = bfs(N, &g);

    if let Some(shortest_travel) = shortest_travel {
        for i in 0..M {
            if shortest_travel.contains(&i) {
                if let Some(ans) = bfs(i, &g) {
                    println!("{}", ans.len());
                } else {
                    println!("-1");
                }
            } else {
                println!("{}", shortest_travel.len());
            }
        }
    } else {
        for _ in 0..M {
            println!("-1");
        }
    }
}
