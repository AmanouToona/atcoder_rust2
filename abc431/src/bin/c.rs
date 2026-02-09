use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, K): (usize, usize, usize),
        mut H: [usize; N],
        mut B: [usize; M],
    }

    H.sort();
    B.sort();
    let mut B: VecDeque<usize> = B.into_iter().collect();
    let mut ans = 0;
    for &h in H.iter() {
        while let Some(b) = B.pop_front() {
            if b >= h {
                ans += 1;
                break;
            }
        }
    }

    if ans >= K {
        println!("Yes");
    } else {
        println!("No");
    }
}
