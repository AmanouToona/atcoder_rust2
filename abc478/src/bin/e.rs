#![allow(non_snake_case)]
use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;
fn main() {
    input! {
        (N, Q): (usize, usize),
        tuv: [(usize, usize, usize); Q],
    }

    let mut is_eq = vec![false; Q];
    let mut frm = vec![HashSet::new(); N];
    let mut to = vec![Vec::new(); N];
    let mut rev = vec![HashSet::new(); N];
    for (i, &(t, u, v)) in tuv.iter().enumerate() {
        let u = u - 1;
        let v = v - 1;

        if t == 0 {
            is_eq[i] = true;
        } else {
            rev[u].push(v);
        }
        frm[v].push(u);
        to[u].push((v, i));
    }

    eprintln!("{:?}", rev);
    eprintln!("{:?}", to);

    // let mut dig = vec![0; N];

    // let mut A = vec![usize::MAX; N];
    // let mut q = VecDeque::new();
    // for i in 0..N {
    //     if frm[i].is_empty() {
    //         q.push(i);
    //         A[i] = 1;
    //     }
    //     dig[i] = frm.len();
    // }

    // while let Some(u) = q.pop_front() {
    //     for &(v, i) in to[u].iter() {
    //         dig[v] -= 1;

    //         if is_eq[i] {
    //             A[v] = A[v].max(A[u]);
    //         } else {
    //             A[v] = A[v].max(A[u] + 1);
    //         }

    //         if dig[v] == 0 {
    //             q.push(v);
    //         }
    //     }
    // }
    // eprintln!("{:?}", A);
    // if A.iter().any(|x| x == &usize::MAX) {
    //     println!("No");
    // } else {
    //     println!("Yes");
    //     println!("{}", A.iter().join(" "));
    // }
}
