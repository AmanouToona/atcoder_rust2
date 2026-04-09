#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        A: [usize; N],
        rx: [(usize, usize); Q],
    }

    let mut rxi: Vec<(usize, usize, usize)> = rx
        .iter()
        .enumerate()
        .map(|(i, &(r, x))| (r - 1, x, i))
        .collect();
    rxi.sort();

    let mut stack = Vec::new();
    let mut ans = vec![0; Q];
    let mut i = 0;
    for &(r, x, q) in rxi.iter() {
        while i <= r {
            let a = A[i];
            if let Some(&last) = stack.last() {
                if last < a {
                    stack.push(a);
                } else if stack[0] >= a {
                    stack[0] = a;
                } else {
                    let mut large = stack.len();
                    let mut small = 0;
                    while large - small > 1 {
                        let mid = (large + small) / 2;
                        if stack[mid] >= a {
                            large = mid;
                        } else {
                            small = mid;
                        }
                    }
                    stack[large] = a;
                }
            } else {
                stack.push(a);
            }

            i += 1;
        }

        let mut small = 0;
        let mut large = stack.len();

        while large - small > 1 {
            let mid = (large + small) / 2;
            if stack[mid] > x {
                large = mid;
            } else {
                small = mid;
            }
        }
        ans[q] = small + 1;
    }

    for i in ans.iter() {
        println!("{i}");
    }
}
