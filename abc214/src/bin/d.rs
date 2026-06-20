#![allow(non_snake_case)]
use ac_library::Dsu;
use proconio::input;
/*
- 主客転倒 ... 辺が何回利用されるか?
- 辺が小さい順に走査
- dsu で管理して、結合したグループのsizeの積の回数用いられる
*/
fn main() {
    input! {
        N: usize,
        mut uvw: [(usize, usize, usize); N - 1],
    }

    uvw.sort_by_key(|x| x.2);
    let mut ans = 0;
    let mut uf = Dsu::new(N);
    for &(u, v, w) in uvw.iter() {
        let u = uf.leader(u - 1);
        let v = uf.leader(v - 1);
        if u == v {
            continue;
        }

        ans += w * uf.size(u) * uf.size(v);
        uf.merge(u, v);
    }

    println!("{ans}");
}
