#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
fn main() {
    input! {
        (A, B, C, D): (usize, usize , usize , usize)
    }

    let n = 4 * 10usize.pow(6);
    let mut fact = vec![mint::new(1); n];
    let mut ifact = vec![mint::new(1); n];

    for i in 1..n {
        fact[i] = fact[i - 1] * mint::new(i);
    }
    ifact[n - 1] = mint::new(1) / fact[n - 1];
    for i in (1..=n - 1).rev() {
        ifact[i - 1] = ifact[i] * mint::new(i);
    }

    let mut ans = mint::new(0);
    for i in 0..=B {
        ans += fact[A + B - i - 1]
            * ifact[A - 1]
            * ifact[B - i]
            * fact[i + D + C]
            * ifact[C]
            * ifact[i + D];
    }

    println!("{}", ans);
}
