#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Ordering;
fn main() {
    input! {
        (N, Q): (usize, usize),
        XY: [(i64, i64); N],
        AB: [(usize, usize); Q],
    }

    let mut points: Vec<(usize, i64, i64)> = XY
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, (x, y))| (i, x, y))
        .collect();

    let cmp = |&x: &(i64, i64), &y: &(i64, i64)| -> Ordering {
        let ah = if (x.0 >= 0 && x.1 >= 0) || (x.0 > 0 && x.1 <= 0) {
            0
        } else {
            1
        };
        let bh = if (y.0 >= 0 && y.1 >= 0) || (y.0 > 0 && y.1 <= 0) {
            0
        } else {
            1
        };

        if ah != bh {
            return ah.cmp(&bh);
        }

        (x.0 * y.1).cmp(&(x.1 * y.0))
    };

    points.sort_by(|&x, &y| cmp(&(x.1, x.2), &(y.1, y.2)));

    let mut pos = vec![0; N];
    for (i, j) in points.iter().enumerate() {
        pos[j.0] = i;
    }

    let mut left = vec![0; N];
    for (i, &v) in points.iter().enumerate().skip(1) {
        let u = points[i - 1];
        if cmp(&(u.1, u.2), &(v.1, v.2)) != Ordering::Equal {
            left[i] = i;
        } else {
            left[i] = left[i - 1]
        }
    }

    let mut right = vec![N - 1; N];
    for (i, &v) in points.iter().enumerate().rev().skip(1) {
        let u = points[i + 1];
        if cmp(&(u.1, u.2), &(v.1, v.2)) != Ordering::Equal {
            right[i] = i;
        } else {
            right[i] = right[i + 1];
        }
    }

    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;
        let pos_u = left[pos[a]];
        let pos_v = right[pos[b]];

        if pos_u <= pos_v {
            let ans = pos_v - pos_u + 1;
            println!("{ans}")
        } else {
            let ans = pos_v + 1 + pos.len() - pos_u;
            println!("{ans}")
        }
    }
}
