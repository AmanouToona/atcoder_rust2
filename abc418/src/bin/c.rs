#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        mut A: [usize; N],
    }

    A.sort();

    let mut cumsum = vec![0; N + 1];
    for (i, &a) in A.iter().enumerate() {
        cumsum[i + 1] = a;
        cumsum[i + 1] += cumsum[i];
    }

    for _ in 0..Q {
        input! {b: usize}

        let pos = A.binary_search(&(b - 1)).unwrap_or_else(|x| x);

        let mut ans = cumsum[pos];
        ans += (b - 1) * (N - pos) + 1;
        if ans > cumsum[N] {
            println!("-1");
        } else {
            println!("{ans}");
        }
    }
}
