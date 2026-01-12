use ac_library::segtree;
use ac_library::Segtree;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        xy: [(i64, i64); N],
    }

    let mut s_min = Segtree::<segtree::Min<i64>>::new(N + 1);
    let mut s_max = Segtree::<segtree::Max<i64>>::new(N + 1);
    let mut t_min = Segtree::<segtree::Min<i64>>::new(N + 1);
    let mut t_max = Segtree::<segtree::Max<i64>>::new(N + 1);

    for (i, &(x, y)) in xy.iter().enumerate() {
        let s = x - y;
        let t = x + y;
        s_min.set(i, s);
        s_max.set(i, s);
        t_min.set(i, t);
        t_max.set(i, t);
    }

    for _ in 0..Q {
        input! {
            query: usize
        }

        match query {
            1 => {
                input! {
                    (i, x, y): (usize, i64, i64)
                }
                let i = i - 1;
                let s = x - y;
                let t = x + y;
                s_min.set(i, s);
                s_max.set(i, s);
                t_min.set(i, t);
                t_max.set(i, t);
            }
            2 => {
                input! {
                    (L, R, x, y): (usize, usize, i64, i64),
                }
                let L = L - 1;
                let R = R - 1;

                let s = x - y;
                let t = x + y;

                let s_min_v = s_min.prod(L..R + 1);
                let s_max_v = s_max.prod(L..R + 1);
                let t_min_v = t_min.prod(L..R + 1);
                let t_max_v = t_max.prod(L..R + 1);

                let ans = s_min_v
                    .abs_diff(s)
                    .max(s_max_v.abs_diff(s))
                    .max(t_min_v.abs_diff(t))
                    .max(t_max_v.abs_diff(t));
                println!("{ans}");
            }
            _ => panic!("wrong query!!"),
        }
    }
}
