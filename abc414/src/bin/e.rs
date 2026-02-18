#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
fn main() {
    input! {
        N: usize,
    }

    let mut ans = mint::new(N) * mint::new(N + 1) / mint::new(2);
    let mut b = 1;
    while b <= N {
        let k = N / b;
        let nxt_b = N / k + 1;
        ans -= k * (nxt_b - b);

        b = nxt_b;
    }

    println!("{ans}");
}
