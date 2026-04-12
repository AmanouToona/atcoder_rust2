#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize;N],
        B: [usize; M],
    }

    let mut eat = vec![0; 2 * 10usize.pow(5) + 1];
    for (&a, i) in A.iter().zip(1..) {
        if eat[a] == 0 {
            eat[a] = i;
            let mut j = a;
            while j < eat.len() {
                if eat[j] != i && eat[j] != 0 {
                    break;
                }
                eat[j] = i;
                j += 1;
            }
        }
    }

    for &b in B.iter() {
        if eat[b] == 0 {
            println!("-1");
        } else {
            println!("{}", eat[b]);
        }
    }
}
