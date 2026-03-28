#![allow(non_snake_case)]
use ac_library::Dsu;
use amplify::confinement::Collection;
use proconio::input;
use std::collections::VecDeque;

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

    for i in 0..N {
        let mut dist = vec![0; N];
        let mut seen = vec![false; N];
        let mut q = VecDeque::new();
        seen[i] = true;
        q.push(i);

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

        for (j, &d) in dist.iter().enumerate() {
            if A[i][j] != d {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
