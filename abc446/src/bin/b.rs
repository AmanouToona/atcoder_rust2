#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut can = vec![true; M + 1];
    'outer: for _ in 0..N {
        input! {L: usize, X:[usize; L]}

        for &x in X.iter() {
            if can[x] {
                println!("{x}");
                can[x] = false;
                continue 'outer;
            }
        }
        println!("0");
    }
}
