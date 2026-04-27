#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        F: [usize; N]
    }

    let mut cnt = vec![0; M];
    for &f in F.iter() {
        cnt[f - 1] += 1;
    }

    if cnt.iter().all(|x| *x < 2) {
        println!("Yes");
    } else {
        println!("No");
    }

    if cnt.iter().all(|x| *x > 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
