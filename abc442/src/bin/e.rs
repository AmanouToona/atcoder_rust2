#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Ordering;
fn main() {
    input! {
        (N, Q): (usize, usize),
        XY: [(i64, i64); N],
        AB: [(usize, usize); Q],
    }

    let cmp = |&(x1, y1): &(i64, i64), &(x2, y2): &(i64, i64)| {
        let d1 = if x1 > 0 || (x1 == 0 && y1 >= 0) { 1 } else { 0 };
        let d2 = if x2 > 0 || (x2 == 0 && y2 >= 0) { 1 } else { 0 };

        if d1.cmp(&d2) != Ordering::Equal {
            return d1.cmp(&d2);
        }

        (y2 * x1).cmp(&(y1 * x2))
    };

    let mut xy: Vec<((i64, i64), usize)> = XY.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    xy.sort_by(|&x, &y| cmp(&x.0, &y.0));

    let mut pref = vec![0];
    let mut num2pref = vec![0; N];

    pref.push(1);
    num2pref[xy[0].1] = 1;

    for ((vi, _), (vj, j)) in xy.iter().zip(xy.iter().skip(1)) {
        if cmp(vi, vj) == Ordering::Equal {
            *pref.last_mut().unwrap() += 1;
        } else {
            pref.push(1);
        }
        num2pref[*j] = pref.len() - 1;
    }
    for i in 0..pref.len() - 1 {
        pref[i + 1] += pref[i];
    }

    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;
        let ans = if num2pref[a] <= num2pref[b] {
            pref[num2pref[b]] - pref[num2pref[a] - 1]
        } else {
            N - (pref[num2pref[a] - 1] - pref[num2pref[b]])
        };
        println!("{ans}");
    }
}
