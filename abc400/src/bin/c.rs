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

        let mut ok = 1;
        let mut ng = 10usize.pow(9) + 1;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;

            if two_a.saturating_mul(mid).saturating_mul(mid) > N {
                ng = mid;
            } else {
                ok = mid;
            }
        }

        ans += ok.div_ceil(2);
    }

    println!("{ans}")
}
