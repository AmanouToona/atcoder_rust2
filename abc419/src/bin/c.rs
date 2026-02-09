#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {N : usize,
        RC: [(usize, usize); N],
    }

    let r_min = RC.iter().map(|&x| x.0).min().unwrap();
    let r_max = RC.iter().map(|&x| x.0).max().unwrap();
    let c_min = RC.iter().map(|&x| x.1).min().unwrap();
    let c_max = RC.iter().map(|&x| x.1).max().unwrap();

    let ans = ((r_max - r_min).div_ceil(2)).max((c_max - c_min).div_ceil(2));
    println!("{ans}");
}
