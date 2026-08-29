#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let mut count_x = vec![0; N + 1];
    for (i, &c) in S.iter().enumerate() {
        if c == 'x' {
            count_x[i + 1] = count_x[i] + 1;
        } else {
            count_x[i + 1] = count_x[i];
        }
    }
    // sentinel
    count_x[N] = usize::MAX;

    let mut ans = 0;
    for i in 1..=N {
        while count_x[ans] < i {
            ans += 1;
        }
        println!("{ans}");
    }
}
