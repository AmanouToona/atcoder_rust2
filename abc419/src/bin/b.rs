#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {Q: usize}

    let mut cnt = vec![0; 101];
    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                input! {x: usize}
                cnt[x] += 1;
            }
            2 => {
                for (i, &v) in cnt.iter().enumerate() {
                    if v != 0 {
                        println!("{i}");
                        cnt[i] -= 1;
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}
