#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        xy: f64
    }
    let x = (xy as i32).unsigned_abs();
    let y = ((xy - x as f64) * 10.) as usize;
    eprintln!("{x} {y}");
    match y {
        0..=2 => {
            println!("{x}-");
        }
        3..=6 => {
            println!("{x}")
        }
        7..=9 => {
            println!("{x}+")
        }
        _ => {}
    }
}
