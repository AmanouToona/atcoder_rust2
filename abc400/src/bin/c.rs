#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
    }

    let mut ans = 0;
    for a in 1..=60 {
        let two_a: usize = 2usize.pow(a);

        if two_a > N {
            break;
        }

        let b_max = (N / two_a).isqrt();
        ans += b_max.div_ceil(2);
    }

    println!("{ans}")
}
