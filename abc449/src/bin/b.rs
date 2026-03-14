#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (H, W, Q): (usize, usize, usize)
    }

    let mut C = W;
    let mut R = H;
    for _ in 0..Q {
        input! {q:usize, rc: usize}
        match q {
            1 => {
                let ans = rc * C;
                R -= rc;
                println!("{ans}");
            }
            2 => {
                let ans = rc * R;
                C -= rc;
                println!("{ans}");
            }
            _ => {
                panic!("")
            }
        }
    }
}
