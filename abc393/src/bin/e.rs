#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }

    let mut a_count = vec![0; 10usize.pow(6) + 1];
    for a in A.iter() {
        a_count[*a] += 1;
    }

    let mut answers = vec![0; 10usize.pow(6) + 1];
    for i in 1..=10usize.pow(6) {
        let mut count = 0;
        let mut j = i;
        while j <= 10usize.pow(6) {
            count += a_count[j];
            j += i;
        }

        if count >= K {
            let mut j = i;
            while j <= 10usize.pow(6) {
                answers[j] = i;
                j += i;
            }
        }
    }

    for a in A.iter() {
        println!("{}", answers[*a]);
    }
}
