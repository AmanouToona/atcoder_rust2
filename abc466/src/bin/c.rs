#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {N: usize}

    let mut cnt = 0;
    let mut left = 0;
    for right in 0..N {
        while left < right {
            println!("? {} {}", left + 1, right + 1);
            input! {judge: String};
            if &judge == "Yes" {
                break;
            } else {
                left += 1;
            }
        }

        cnt += right - left;
    }
    println!("! {cnt}");
}
