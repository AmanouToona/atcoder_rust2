#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {S:Chars}

    let mut ans = 0_f64;
    for (i, &s) in S.iter().enumerate() {
        if s != 't' {
            continue;
        }

        let mut cnt = 0.;

        for (j, &end) in S.iter().skip(i).enumerate() {
            let j = j + 1;
            if end == 't' {
                cnt += 1.;
                if j >= 3 {
                    ans = ans.max((cnt - 2.) / (j - 2) as f64);
                }
            }
        }
    }

    println!("{ans}");
}
