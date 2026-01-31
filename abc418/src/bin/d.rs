#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        _: usize,
        T: Chars,
    }

    let mut u = vec![0, 0];
    let mut ans: usize = 0;
    for &t in T.iter() {
        let mut v = vec![0, 0];
        if t == '0' {
            v[0] += 1;
            v[0] += u[1];
            v[1] += u[0];
        } else {
            v[1] += 1;
            v[1] += u[1];
            v[0] += u[0];
        }
        u = v;
        ans += u[1];
    }

    println!("{ans}");
}
