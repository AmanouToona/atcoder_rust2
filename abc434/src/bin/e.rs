use ac_library::Dsu;
use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        xr: [(i64, i64); N],
    }

    let mut dsu = Dsu::new(2 * N);
    let mut i2n = HashMap::new(); // i64 -> node number
    let mut has_cycle = vec![false; 2 * N];

    for &(x, r) in xr.iter() {
        let left = x - r;
        let right = x + r;

        if !i2n.contains_key(&left) {
            let len = i2n.len();
            i2n.insert(left, len);
        }
        if !i2n.contains_key(&right) {
            let len = i2n.len();
            i2n.insert(right, len);
        }

        let &left = i2n.get(&left).unwrap();
        let &right = i2n.get(&right).unwrap();

        if dsu.same(left, right) {
            has_cycle[dsu.leader(left)] = true;
        } else {
            let leader_left = dsu.leader(left);
            let leader_right = dsu.leader(right);
            dsu.merge(left, right);
            has_cycle[dsu.leader(left)] = has_cycle[leader_left] || has_cycle[leader_right];
        }
    }

    let mut ans = 0;
    for i in dsu.groups() {
        let leader = dsu.leader(i[0]);

        if has_cycle[leader] {
            ans += i.len();
        } else {
            ans += i.len() - 1;
        }
    }
    println!("{ans}");
}
