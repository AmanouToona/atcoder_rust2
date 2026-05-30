#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut AB: [(usize, usize); N],
    }
    AB.sort_by(|&i, &j| j.1.cmp(&i.1));
    AB.sort_by_key(|x| x.0);

    let mut q = Vec::new();
    for &(_, b) in AB.iter() {
        if q.is_empty() {
            q.push(b);
            continue;
        }
        if let Some(last) = q.last() {
            if *last < b {
                q.push(b);
                continue;
            }
        }

        let idx = q.partition_point(|x| *x < b);
        q[idx] = b;
    }

    println!("{}", q.len());
}
