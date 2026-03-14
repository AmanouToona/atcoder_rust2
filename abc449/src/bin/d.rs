#![allow(non_snake_case)]
use num::Integer;
use proconio::input;
fn cumsum(L: i64, R: i64, d: i64) -> i64 {
    let n = (R - L) / d + 1;
    (L + R) * n / 2
}

fn square(x: i64) -> usize {
    let x: usize = x.abs() as usize;
    let x = if x.is_odd() { x - 1 } else { x };
    let x = x + 1;
    let n = cumsum(1, x as i64, 2) as usize * 2;
    n - (x + 2) / 2
}

fn rectangle(x: usize, y: usize) -> usize {
    let (x, y) = if x < y { (y, x) } else { (x, y) };

    let x = if x.is_odd() { x - 1 } else { x };
    let y_even = if y.is_odd() { y - 1 } else { y };

    let mut res = square(y_even as i64);
    // eprintln!("{res} {y} {x}");
    res += (y + 1) * (x - y_even) / 2;
    res
}

fn rectangle2(x1: usize, x2: usize, y1: usize, y2: usize) -> usize {
    if x1 > x2 || y1 > y2 {
        return 0;
    }

    let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    let mut res = rectangle(x2, y2);
    if x1 > 0 {
        res -= rectangle(x1 - 1, y2);
    }
    if y1 > 0 {
        res -= rectangle(x2, y1 - 1);
    }

    if y1 > 0 && x1 > 0 {
        res += rectangle(x1 - 1, y1 - 1);
    }

    res
}

fn main() {
    input! {
        (L, R, D, U): (i64, i64, i64, i64),
    }

    // eprintln!("{}", rectangle(3, 4));
    // eprintln!("{}", rectangle2(0, 3, 1, 4));

    let mut ans = 0;

    // 1
    if R > 0 && U > 0 {
        let r = R.abs() as usize;
        let l = L.max(1) as usize;
        let u = U.abs() as usize;
        let d = D.max(1) as usize;

        ans += rectangle2(l, r, d, u);
        eprintln!("{ans}")
    }

    // 2
    if L < 0 && U > 0 {
        let l = L.abs() as usize;
        let r = R.min(-1).abs() as usize;
        let u = U as usize;
        let d = D.max(1) as usize;

        ans += rectangle2(r, l, d, u);
        eprintln!("{ans}")
    }

    // 3
    if L < 0 && D < 0 {
        let r = R.min(-1).abs() as usize;
        let l = L.abs() as usize;
        let d = D.abs() as usize;
        let u = U.min(-1).abs() as usize;
        ans += rectangle2(r, l, u, d);
        eprintln!("{ans}")
    }

    // 4
    if R > 0 && D < 0 {
        let r = R.abs() as usize;
        let l = L.max(1) as usize;
        let d = D.abs() as usize;
        let u = U.min(-1).abs() as usize;

        ans += rectangle2(l, r, u, d);
        eprintln!("{ans}")
    }

    // x 軸
    if D <= 0 && U >= 0 {
        let len = (R - L).abs() as usize;
        if L % 2 == 0 {
            ans += 1 + len / 2;
        } else {
            ans += (len + 1) / 2;
        }
        eprintln!("{ans}")
    }

    // y 軸
    if L <= 0 && R >= 0 {
        let len = (U - D).abs() as usize;
        if D % 2 == 0 {
            ans += 1 + len / 2;
        } else {
            ans += (len + 1) / 2;
        }
    }

    // 原点
    if (L <= 0 && R >= 0) && (D <= 0 && U >= 0) {
        ans -= 1;
    }

    println!("{ans}");
}
