#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let S: Vec<usize> = S
        .iter()
        .map(|&x| (x.to_digit(10).unwrap()) as usize)
        .collect();

    let mut left = vec![0; N]; // 左側に移動するためのコスト
    let mut cost = 0;
    for (i, &s) in S.iter().enumerate() {
        if s == 1 {
            cost += 1;
        } else {
            left[i] = cost;
        }
    }

    let mut right = vec![0; N]; // 右側に移動するためのコスト
    cost = 0;
    for (i, &s) in S.iter().enumerate().rev() {
        if s == 1 {
            cost += 1;
        } else {
            right[i] = cost;
        }
    }

    let mut ans: usize = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        ans += l.min(r);
    }
    eprintln!("{:?}", left);
    eprintln!("{:?}", right);
    println!("{ans}");
}
