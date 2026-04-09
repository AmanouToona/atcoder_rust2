#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        Q: [usize; N],
        A: [usize; N],
        B: [usize; N],
    }

    let mut ans = 0;
    'outer: for a in 0..=1000_000 {
        let mut b_can = 1 << 60;
        for n in 0..N {
            if Q[n] < a * A[n] {
                continue 'outer;
            }
            if B[n] == 0 {
                continue;
            }
            b_can = b_can.min((Q[n] - a * A[n]) / B[n]);
        }

        ans = ans.max(b_can + a);
    }

    println!("{ans}");
}
