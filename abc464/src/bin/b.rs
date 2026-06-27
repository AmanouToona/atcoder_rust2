#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (H, W) : (usize, usize),
        C: [Chars; H],
    }

    let mut top = 0;
    let mut buttom = H - 1;
    let mut left = 0;
    let mut right = W - 1;

    while C[top]
        .iter()
        .skip(left)
        .take(right + 1 - left)
        .all(|&c| c == '.')
    {
        top += 1;
    }

    while C[buttom]
        .iter()
        .skip(left)
        .take(right + 1 - left)
        .all(|&c| c == '.')
        && buttom > top
    {
        buttom -= 1;
    }
    // eprintln!("{top} {buttom}");

    'outer: loop {
        for h in top..=buttom {
            if C[h][left] != '.' {
                break 'outer;
            }
        }
        left += 1;
    }

    'outer: loop {
        for h in top..=buttom {
            if C[h][right] != '.' {
                break 'outer;
            }
        }
        right -= 1;
    }

    for s in C.iter().skip(top).take(buttom + 1 - top) {
        let ans = &s.iter().skip(left).take(right + 1 - left).join("");
        println!("{ans}");
    }
}
