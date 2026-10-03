#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, V): (usize, usize),
        W: [usize; N],
    }

    let mut ans = 0;
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                if i + j + k + 3 <= V {
                    ans = ans.max(W[i] + W[j] + W[k]);
                }
            }
        }
    }
    println!("{ans}");
}
