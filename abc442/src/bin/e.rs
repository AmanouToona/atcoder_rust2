#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        XY: [(i64, i64); N],
        AB: [(usize, usize); Q],
    }

    let mut v0 = Vec::new();
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();

    for (i, &(x, y)) in XY.iter().enumerate() {
        if x >= 0 {
            if y >= 0 {
                v0.push((i, x, y));
            } else {
                v1.push((i, x, y));
            }
        } else {
            if y <= 0 {
                v2.push((i, x, y));
            } else {
                v3.push((i, x, y));
            }
        }
    }

    v0.sort_by(|&x, &y| (x.1 * y.2).cmp(&(x.2 * y.1)));
    v1.sort_by(|&x, &y| (x.1 * y.2).cmp(&(x.2 * y.1)));
    v2.sort_by(|&x, &y| (x.1 * y.2).cmp(&(x.2 * y.1)));
    v3.sort_by(|&x, &y| (x.1 * y.2).cmp(&(x.2 * y.1)));

    let vec: Vec<(usize, i64, i64)> = v0
        .iter()
        .chain(v1.iter())
        .chain(v2.iter())
        .chain(v3.iter())
        .cloned()
        .collect();

    let mut pos = vec![0; N];
    for (i, &j) in vec.iter().enumerate() {
        pos[j.0] = i
    }

    let mut left = vec![0; N];
    let mut l = 0;
    for (i, &j) in vec.iter().enumerate().skip(1) {
        let u = vec[i - 1];
        if j.1 * u.2 != j.2 * u.1 {
            l = i
        }
        left[i] = l;
    }

    let mut right = vec![N - 1; N];
    let mut r = N - 1;
    for (i, &j) in vec.iter().enumerate().rev().skip(1) {
        let u = vec[i + 1];
        if j.1 * u.2 != j.2 * u.1 {
            r = i
        }
        right[i] = r;
    }
    // println!("{:?}", vec);
    eprintln!("{:?}", left);
    eprintln!("{:?}", right);

    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;
        let pos_u = left[pos[a]];
        let pos_v = right[pos[b]];

        if pos_u <= pos_v {
            let ans = pos_v - pos_u + 1;
            println!("{ans}")
        } else {
            let ans = pos_v + 1 + vec.len() - pos_u;
            println!("{ans}")
        }
    }
}
