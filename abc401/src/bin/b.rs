#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {N: usize}

    let mut is_authenticated = false;
    let mut ans = 0;
    for _ in 0..N {
        input! {S: String}
        match &S[..] {
            "login" => {
                is_authenticated = true;
            }
            "logout" => {
                is_authenticated = false;
            }
            "public" => {}
            "private" => {
                if !is_authenticated {
                    ans += 1;
                }
            }
            _ => {
                panic!("wrong")
            }
        }
    }
    println!("{ans}");
}
