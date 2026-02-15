#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {T: usize}
    for _ in 0..T {
        input! {
            (N, M): (usize, usize),
            mut A: [usize; N],
            mut B: [usize; N],
        }

        A = A.iter().map(|&x| x % M).collect();
        A.sort();

        B = B.iter().map(|&x| x % M).collect();
        B.sort_by(|&x, &y| y.cmp(&x));

        let mut cnt_m = 0;
        let mut i = 0;
        for &b in B.iter() {
            while i < N && A[i] + b < M {
                i += 1
            }

            if i >= N {
                break;
            }

            if A[i] + b >= M {
                cnt_m += 1;
            }
            i += 1;
        }

        let ans: usize = A.iter().sum::<usize>() + B.iter().sum::<usize>() - M * cnt_m;
        println!("{ans}");
    }
}
