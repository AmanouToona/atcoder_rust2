#![allow(non_snake_case)]
use ac_library::Dsu;
use amplify::confinement::Collection;
use proconio::input;
use std::{collections::VecDeque, usize};

// Lowest Common Ancestor
struct LCA {
    parents: Vec<Vec<usize>>, // parents[k][u] := u の 2 ** k 先の親
    distance: Vec<usize>,     // root からの距離
}

impl LCA {
    fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut k = 0;
        while (1 << k) <= n {
            k += 1
        }

        let mut q = VecDeque::new();
        let mut parents = vec![vec![0; n]; k];
        let mut distance = vec![usize::MAX; n];
        q.push_back(0);
        distance[0] = 0;

        while let Some(u) = q.pop_front() {
            for &v in g[u].iter() {
                if distance[v] == usize::MAX {
                    parents[0][v] = u;
                    q.push_back(v);

                    distance[v] = distance[u] + 1;
                }
            }
        }

        if !distance.iter().skip(1).all(|x| *x > 0) {
            panic!("non connected tree !!")
        }

        for i in 0..k - 1 {
            for u in 0..n {
                parents[i + 1][u] = parents[i][parents[i][u]];
            }
        }

        LCA { parents, distance }
    }

    fn query(&self, u: usize, v: usize) -> usize {
        let mut u = u;
        let mut v = v;
        let k = self.parents.len();

        if self.distance[u] > self.distance[v] {
            std::mem::swap(&mut u, &mut v);
        }

        // 深さを揃える
        for i in 0..k {
            if (self.distance[v] - self.distance[u]) >> i & 1 == 1 {
                v = self.parents[i][v];
            }
        }

        if u == v {
            return u;
        }

        // 共通祖先
        for i in (0..k).rev() {
            if self.parents[i][u] != self.parents[i][v] {
                u = self.parents[i][u];
                v = self.parents[i][v];
            }
        }

        self.parents[0][v]
    }
}

fn main() {
    input! {
        N: usize,
    }

    let mut A = vec![vec![0; N]; N];
    let mut edge: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..N - 1 {
        input! {a: [usize; N - 1 - i]}
        for (j, aa) in a.iter().enumerate() {
            A[i][i + j + 1] = *aa;
            A[i + j + 1][i] = *aa;
            edge.push((*aa, i, i + j + 1));
        }
    }
    edge.sort();

    let mut dsu = Dsu::new(N);
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); N];
    for (c, i, j) in edge.iter() {
        if dsu.same(*i, *j) {
            continue;
        } else {
            g[*i].push(*j);
            g[*j].push(*i);
            dsu.merge(*i, *j);
        }
    }

    // 根を 0 とした距離を算出
    let mut dist = vec![0; N];
    let mut seen = vec![false; N];
    let mut q = VecDeque::new();
    seen[0] = true;
    q.push(0);

    while let Some(u) = q.pop_front() {
        for &v in g[u].iter() {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            dist[v] = dist[u] + A[u][v];
            q.push(v);
        }
    }

    let lca = LCA::new(&g);
    for i in 0..N {
        for j in i + 1..N {
            let k = lca.query(i, j);
            let d = (dist[i] - dist[k]) + (dist[j] - dist[k]);
            if A[i][j] != d {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
