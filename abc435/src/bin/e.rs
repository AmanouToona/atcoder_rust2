use proconio::input;
use std::collections::BTreeMap;
#[allow(non_snake_case)]
fn main() {
    input! {(N, Q): (usize, usize)}

    let mut tree: BTreeMap<usize, usize> = BTreeMap::new();

    let mut ans = N;
    for _ in 0..Q {
        input! {(L, R) : (usize, usize)}

        let mut nxt_l = L;
        let mut nxt_r = R;
        let mut remove = Vec::new();
        let (&start_L, _) = tree.range(0..=L).next_back().unwrap_or((&L, &0));
        for (&l, &r) in tree.range(start_L..=R) {
            if r < L {
                continue;
            }
            remove.push((l, r));
            nxt_l = nxt_l.min(l);
            nxt_r = nxt_r.max(r);
        }

        for &(l, r) in remove.iter() {
            ans += r - l + 1;
            tree.remove(&l);
        }
        ans -= nxt_r - nxt_l + 1;
        tree.insert(nxt_l, nxt_r);

        println!("{ans}");
    }
}
