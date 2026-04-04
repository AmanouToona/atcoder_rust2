#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut left = vec![0; N + 1];
    let mut set = std::collections::HashSet::new();
    for (i, &a) in A.iter().enumerate() {
        set.insert(a);
        left[i + 1] = set.len();
    }

    let mut right = vec![0; N + 1];
    let mut set = std::collections::HashSet::new();
    for (i, &a) in A.iter().enumerate().rev() {
        set.insert(a);
        right[i] = set.len();
    }

    let mut ans = 0;
    for i in 1..N {
        ans = ans.max(left[i] + right[i]);
    }

    println!("{ans}");

    // eprintln!("{:?}", left);
    // eprintln!("{:?}", right);
}
