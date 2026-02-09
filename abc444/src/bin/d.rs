use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }

    A.sort();
    let max = A[N - 1];
    let mut ans = Vec::new();
    let mut sum = 0;
    let mut left = 0;
    for n in 1..=max {
        // n 以上の A の個数を知りたい
        while left < N && A[left] < n {
            left += 1;
        }

        sum += N - left;
        ans.push(sum % 10);
        sum /= 10;
    }

    while sum != 0 {
        ans.push(sum % 10);
        sum /= 10;
    }

    let ans: String = ans.iter().rev().join("");
    println!("{ans}");
}
