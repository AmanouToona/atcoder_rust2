use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut ans: Vec<usize> = (0..N).collect();

    for (i, &a) in A.iter().enumerate().rev() {
        ans[i] = ans[a - 1];
    }

    let ans: String = ans.iter().map(|&x| x + 1).join(" ");
    println!("{ans}");
}
