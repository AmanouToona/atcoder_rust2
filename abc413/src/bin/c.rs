#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        Q: usize
    }

    let mut que: VecDeque<(usize, usize)> = VecDeque::new();

    for _ in 0..Q {
        input! {
            q: usize,
        }

        match q {
            1 => {
                input! {(c, x): (usize, usize)}
                que.push_back((c, x));
            }
            2 => {
                input! {mut k: usize}
                let mut ans = 0;
                while k > 0 {
                    let (c, x) = que.pop_front().unwrap();
                    let d = k.min(c);
                    ans += x * d;
                    if d < c {
                        que.push_front((c - d, x));
                    }
                    k -= d;
                }
                println!("{ans}");
            }
            _ => {}
        }
    }
}
