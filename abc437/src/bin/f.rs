use ac_library::segtree::Monoid;
use ac_library::Segtree;
use proconio::input;

#[derive(Clone)]
struct Node {
    s_min: i64,
    s_max: i64,
    t_min: i64,
    t_max: i64,
}

impl Node {
    fn new(x: i64, y: i64) -> Self {
        Node {
            s_min: x - y,
            s_max: x - y,
            t_min: x + y,
            t_max: x + y,
        }
    }
}

struct M;
impl Monoid for M {
    type S = Node;
    fn identity() -> Self::S {
        Node {
            s_min: i64::MAX,
            s_max: i64::MIN,
            t_min: i64::MAX,
            t_max: i64::MIN,
        }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        Node {
            s_min: a.s_min.min(b.s_min),
            s_max: a.s_max.max(b.s_max),
            t_min: a.t_min.min(b.t_min),
            t_max: a.t_max.max(b.t_max),
        }
    }
}
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        xy: [(i64, i64); N],
    }

    let st: Vec<Node> = xy.iter().map(|(x, y)| Node::new(*x, *y)).collect();
    let mut seg = Segtree::<M>::from(st);

    for (i, &(x, y)) in xy.iter().enumerate() {
        seg.set(i, Node::new(x, y));
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
                seg.set(i, Node::new(x, y));
            }
            2 => {
                input! {
                    (L, R, x, y): (usize, usize, i64, i64),
                }
                let L = L - 1;
                let R = R - 1;

                let s = x - y;
                let t = x + y;

                let node = seg.prod(L..R + 1);
                let ans = s
                    .abs_diff(node.s_min)
                    .max(s.abs_diff(node.s_max))
                    .max(t.abs_diff(node.t_min))
                    .max(t.abs_diff(node.t_max));

                println!("{ans}");
            }
            _ => panic!("wrong query!!"),
        }
    }
}
