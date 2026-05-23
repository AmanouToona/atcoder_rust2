#![allow(non_snake_case)]
use proconio::input;

/*
1


*/
fn main() {
    input! {
        (N, Q): (usize, usize),
    }

    for _ in 0..Q {
        input! {q: usize }
        match q {
            1 => {
                input! {x : usize}
            }
            2 => {
                input! {y: usize}
            }
            _ => {
                panic!()
            }
        }
    }
}
