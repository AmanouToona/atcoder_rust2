#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        A: [[usize; 6]; 3],
    }

    let mut ans: f64 = 0.0;
    for &i in A[0].iter() {
        for &j in A[1].iter() {
            for &k in A[2].iter() {
                if [i, j, k].contains(&4) && [i, j, k].contains(&5) && [i, j, k].contains(&6) {
                    ans += 1.;
                }
            }
        }
    }

    let ans = ans / (6. * 6. * 6.);
    println!("{ans}");
}
