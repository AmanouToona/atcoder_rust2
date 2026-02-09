#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        Q: usize,
    }

    let mut volume = 0;
    let mut play = false;

    for _ in 0..Q {
        input! {a: usize}

        if a == 1 {
            volume += 1;
        } else if a == 2 {
            volume = if volume <= 1 { 0 } else { volume - 1 };
        } else {
            play = !play;
        }

        if play && volume >= 3 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
