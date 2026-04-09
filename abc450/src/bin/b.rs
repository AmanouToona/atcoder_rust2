#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
    }

    let mut C = vec![Vec::new(); N - 1];
    for i in 0..=N - 2 {
        input! {
            c: [usize; N  - 1 - i],
        }
        C[i] = c;
    }

    for a in 0..=N - 2 {
        for c in a + 2..N {
            for b in a + 1..c {
                // eprintln!("a{a} b{b} c{c}");
                let ab = C[a][b - a - 1];
                let bc = C[b][c - b - 1];
                let ac = C[a][c - a - 1];
                if ab + bc < ac {
                    println!("Yes");
                    return;
                }
                // eprintln!("{ab} {bc} {ac}");
            }
        }
    }
    println!("No");
}
