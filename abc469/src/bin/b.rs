#![allow(non_snake_case)]
use itertools::izip;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        _: usize,
        S: Chars,
    }

    let mut count = 0;
    for (pre, now, nxt) in izip!(
        ['x'].iter().chain(S.iter()),
        S.iter(),
        S.iter().skip(1).chain(['x'].iter())
    ) {
        if *pre == 'x' && *now == 'x' && *nxt == 'x' {
            count += 1;
        }
    }
    println!("{count}");
}
