#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [i64; N],
    }

    let mut mA = Vec::new();
    let mut pA = Vec::new();
    for &a in A.iter() {
        if a < 0 {
            mA.push(a);
        } else {
            pA.push(a);
        }
    }

    mA.sort();
    pA.sort_by(|x, y| y.cmp(x));

    let mut ans = 0;
    let mut now = 0;
    while !mA.is_empty() || !pA.is_empty() {
        let nxt = if mA.is_empty() {
            pA.pop().unwrap()
        } else if pA.is_empty() {
            mA.pop().unwrap()
        } else if pA.last().unwrap().abs_diff(now) < mA.last().unwrap().abs_diff(now) {
            pA.pop().unwrap()
        } else {
            mA.pop().unwrap()
        };

        ans += now.abs_diff(nxt);
        now = nxt;
    }

    println!("{ans}");
}
