#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
絶対値を外す
各数字が、何回 +, - として出現するかをカウント
*/

fn main() {
    input! {
        (N, M):(usize, usize),
        mut A: [usize; N],
        mut B: [usize; M],
    }
    A.sort();
    B.sort();

    let mut pls_a = vec![0; N];
    let mut iter_b = B.iter().peekable();
    let mut i = 0;
    for (j, &a) in A.iter().enumerate() {
        while i < M {
            if let Some(b) = iter_b.peek() {
                if **b <= a {
                    i += 1;
                    iter_b.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        pls_a[j] = i;
    }

    let mut pls_b = vec![0; M];
    let mut iter_a = A.iter().peekable();
    let mut j = 0;
    for (i, &b) in B.iter().enumerate() {
        while j < N {
            if let Some(&&a) = iter_a.peek() {
                if a < b {
                    j += 1;
                    iter_a.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        pls_b[i] = j;
    }

    let mut ans = mint::new(0);
    for (&a, &c) in A.iter().zip(pls_a.iter()) {
        ans += mint::new(a) * c;
        ans -= mint::new(a) * (M - c);
    }

    for (&b, &c) in B.iter().zip(pls_b.iter()) {
        ans += mint::new(b) * c;
        ans -= mint::new(b) * (N - c);
    }
    println!("{ans}");
}
