#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (H, W, Q): (usize, usize, usize),
        rcx: [(usize, usize, char); Q],
    }

    let mut board = vec![vec!['A'; W]; H];
    let mut foot = vec![vec![false; W]; H];

    for &(R, C, x) in rcx.iter().rev() {
        for r in (0..R).rev() {
            let mut changed = false;
            for c in (0..C).rev() {
                if foot[r][c] {
                    break;
                }
                foot[r][c] = true;
                changed = true;
                board[r][c] = x;
            }
            if !changed {
                break;
            }
        }
    }

    for i in board.iter() {
        let ans: String = i.iter().join("");
        println!("{ans}");
    }
}
