#![allow(non_snake_case)]
use ac_library::Dsu;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (N, Q): (usize, usize),
        uv: [(usize, usize); Q],
    }

    let mut dsu = Dsu::new(N);
    let mut color: Vec<[HashSet<usize>; 2]> = vec![[HashSet::new(), HashSet::new()]; N];

    for (i, c) in color.iter_mut().enumerate() {
        c[0].insert(i);
    }

    let mut ans = 0;
    for (u, v) in uv.iter() {
        let mut u = u - 1;
        let mut v = v - 1;

        // u が大きい
        if dsu.size(u) < dsu.size(v) {
            std::mem::swap(&mut u, &mut v);
        }

        let root_u = dsu.leader(u);
        let root_v = dsu.leader(v);

        let is_same_color = !((color[root_u][0].contains(&u)) ^ (color[root_v][0].contains(&v)));
        if dsu.same(u, v) {
            if is_same_color {
                ans = -1;
            }
        } else {
            ans -= color[root_u].iter().map(|x| x.len()).min().unwrap() as i64;
            ans -= color[root_v].iter().map(|x| x.len()).min().unwrap() as i64;

            let from = std::mem::take(&mut color[root_v]);
            color[root_u][0].extend(&from[is_same_color as usize]);
            color[root_u][1].extend(&from[1 - is_same_color as usize]);

            dsu.merge(root_u, root_v);
            let new_root = dsu.leader(root_u);
            color[new_root] = color[root_u].clone();

            ans += color[new_root].iter().map(|x| x.len()).min().unwrap() as i64;
        }
        println!("{ans}");
    }
}
