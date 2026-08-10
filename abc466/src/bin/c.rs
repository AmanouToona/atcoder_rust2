#![allow(non_snake_case)]
use proconio::input_interactive;
fn main() {
    input_interactive!(N: usize);

    let mut left = 1;
    let mut ans = 0;
    for right in 1..=N {
        while left < right {
            println!("? {left} {right}");
            input_interactive!(yn: String);
            let yn = yn.as_str();

            match yn {
                "No" => {
                    left += 1;
                }
                _ => {
                    ans += right - left;
                    break;
                }
            }
        }
    }
    println!("! {ans}");
}
