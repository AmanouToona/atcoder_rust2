#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut hight = vec![0; N];
    let mut reach = vec![0; Q + 1];
    let mut remove_cnt = 0;

    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                input! {x: usize}
                let x = x - 1;
                hight[x] += 1;
                reach[hight[x]] += 1;

                if reach[hight[x]] == N {
                    remove_cnt = hight[x];
                }
            }
            2 => {
                input! { y : usize}
                if remove_cnt + y > Q {
                    println!("0")
                } else {
                    println!("{}", reach[y + remove_cnt]);
                }
            }
            _ => {
                panic!()
            }
        }
    }
}
