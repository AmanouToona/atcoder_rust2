#![allow(non_snake_case)]
use ac_library::LazySegtree;
use ac_library::{MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    input! {
        (N, Q): (usize, usize),
        LRX: [(usize, usize, usize); Q],
    }

    let mut ope_add = vec![Vec::new(); N + 1];
    let mut ope_sub = vec![Vec::new(); N + 1];

    for &(l, r, x) in LRX.iter() {
        ope_add[l - 1].push(x);
        ope_sub[r].push(x);
    }

    let mut now: HashMap<usize, usize> = HashMap::new();
    let mut ans = Vec::new();

    for i in 0..N {
        for &j in ope_add[i].iter() {
            *now.entry(j).or_default() += 1;
        }
        for &j in ope_sub[i].iter() {
            *now.entry(j).or_default() -= 1;
            if now[&j] == 0 {
                now.remove(&j);
            }
        }
        ans.push(now.len());
    }

    println!("{}", ans.iter().join(" "));

    // struct M;
    // impl Monoid for M {
    //     type S = HashSet<usize>;
    //     fn identity() -> Self::S {
    //         HashSet::new()
    //     }
    //     fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
    //         HashSet::new()
    //     }
    // }

    // struct F;
    // impl MapMonoid for F {
    //     type M = M;
    //     type F = HashSet<usize>;
    //     fn composition(f: &Self::F, g: &Self::F) -> Self::F {
    //         let mut res = g.clone();
    //         res.extend(f);
    //         res
    //     }
    //     fn identity_map() -> Self::F {
    //         HashSet::new()
    //     }
    //     fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
    //         let mut res = x.clone();
    //         res.extend(f);
    //         res
    //     }
    // }

    // let mut seg = LazySegtree::<F>::new(N);
}
