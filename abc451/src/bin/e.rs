#![allow(non_snake_case)]
use ac_library::Dsu;
use amplify::confinement::Collection;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        N: usize,
    }

    let mut A = vec![vec![0; N]; N];
    let mut edge: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
    for i in 0..N - 1 {
        input! {a: [usize; N - 1 - i]}
        for (j, aa) in a.iter().enumerate() {
            A[i][i + j + 1] = *aa;
            A[i + j + 1][i] = *aa;
            edge.push((Reverse(*aa), i, j));
        }
    }

    let mut dsu = Dsu::new(N);
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); N];
    for (Reverse(c), i, j) in edge.iter() {
        if dsu.same(*i, *j) {
            continue;
        } else {
            g[*i].push(*j);
            g[*j].push(*i);
        }
    }

    let mut parent: Vec<Vec<Option<usize>>> = vec![vec![None; N]; 13]; // parent[k][u]: u の 2**k おや
    let mut dist: Vec<usize> = vec![0; N];
    let mut seen = vec![false; N];
    let mut q = VecDeque::new();
    q.push(0);
    seen[0] = true;
    while let Some(u) = q.pop_front() {
        for &v in g[u].iter() {
            if seen[v] {
                continue;
            }
            q.push(v);
            seen[v] = true;
            parent[0][v] = Some(u);
            dist[v] = dist[u] + A[u][v];
        }
    }

    for i in 0..12 {
        for v in 0..N {
            if parent[i][v].is_some() {
                parent[i + 1][v] = parent[i][parent[i][v].unwrap()]
            }
        }
    }

    for i in parent.iter() {
        eprintln!("{:?}", i);
    }

    // let mut B: Vec<Vec<Option<usize>>> = vec![vec![None; N]; N];
    // let mut q = VecDeque::new();
}
