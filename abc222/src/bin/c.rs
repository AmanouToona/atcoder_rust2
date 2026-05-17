#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [Chars; N  * 2],
    }

    let mut win_count = Vec::new();
    for i in 0..2 * N {
        win_count.push((0, i));
    }

    for i in 0..M {
        for j in (0..2 * N).step_by(2) {
            let p1 = win_count[j].1;
            let p2 = win_count[j + 1].1;

            match (A[p1][i], A[p2][i]) {
                ('G', 'P') => {
                    win_count[j + 1].0 += 1;
                }
                ('G', 'C') => {
                    win_count[j].0 += 1;
                }
                ('C', 'P') => {
                    win_count[j].0 += 1;
                }
                ('C', 'G') => {
                    win_count[j + 1].0 += 1;
                }
                ('P', 'C') => {
                    win_count[j + 1].0 += 1;
                }
                ('P', 'G') => {
                    win_count[j].0 += 1;
                }
                _ => {}
            }
        }
        win_count.sort_by(|x, y| x.1.cmp(&y.1));
        win_count.sort_by(|x, y| y.0.cmp(&x.0));
    }

    for (_, i) in win_count.iter() {
        println!("{}", i + 1);
    }
}
