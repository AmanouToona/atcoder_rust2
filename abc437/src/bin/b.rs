use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, N): (usize, usize, usize),
        A: [[usize; W]; H],
        B: [usize; N],
    }

    let B: HashSet<usize> = HashSet::from_iter(B);
    let mut ans = 0;

    for h in 0..H {
        let mut cnt_row = 0;
        for a in A[h].iter() {
            if B.contains(a) {
                cnt_row += 1;
            }
        }
        ans = ans.max(cnt_row);
    }
    println!("{ans}");
}
