#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Ordering;
fn main() {
    input! {
        (N, Q): (usize, usize),
        XY: [(i64, i64); N],
        AB: [(usize, usize); Q],
    }

    let cmp = |x: &(i64, i64), y: &(i64, i64)| -> Ordering {
        let xh = if x.0 > 0 && x.1 >= 0 || (x.0 >= 0 && x.1 <= 0) {
            0
        } else {
            1
        };
        let yh = if y.0 > 0 && y.1 >= 0 || (y.0 >= 0 && y.1 <= 0) {
            0
        } else {
            1
        };

        if xh != yh {
            return xh.cmp(&yh);
        };

        (x.0 * y.1).cmp(&(x.1 * y.0))
    };

    let mut xyi: Vec<(i64, i64, usize)> = XY
        .iter()
        .enumerate()
        .map(|(i, xy)| (xy.0, xy.1, i))
        .collect();
    xyi.sort_by(|x, y| cmp(&(x.0, x.1), &(y.0, y.1)));

    let mut pref = vec![0, 1];
    let mut num2pref = vec![0; N];
    num2pref[xyi[0].2] = 1;

    for (&(ux, uy, _), &(x, y, i)) in xyi.iter().zip(xyi.iter().skip(1)) {
        if cmp(&(ux, uy), &(x, y)) == Ordering::Equal {
            *pref.last_mut().unwrap() += 1;
        } else {
            pref.push(1);
        }
        num2pref[i] = pref.len() - 1;
    }
    for i in 0..pref.len() - 1 {
        pref[i + 1] += pref[i]
    }
    // eprintln!("{:?}", xyi);
    // eprintln!("{:?}", num2pref);
    // eprintln!("{:?}", pref);

    for &(a, b) in AB.iter() {
        let a = num2pref[a - 1];
        let b = num2pref[b - 1];

        let ans = if b >= a {
            pref[b] - pref[a - 1]
        } else {
            N - (pref[a - 1] - pref[b])
        };

        println!("{ans}");
    }
}
