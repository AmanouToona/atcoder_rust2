#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut ans = Vec::from_iter(A.iter().cloned().take(2));

    for &a in A.iter().skip(2) {
        ans.push(a);
        ans.sort_by(|x, y| y.cmp(x));
        println!("{}", ans[2]);
        if ans.len() > 3 {
            ans.pop();
        }
    }
}
