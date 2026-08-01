#![allow(non_snake_case)]

use proconio::input;

fn search(num: &mut Vec<usize>, used: &mut Vec<bool>, P: &Vec<usize>, Q: &Vec<usize>) -> usize {
    let n = used.len();

    if num.len() == n {
        if num == P || num == Q {
            return 0;
        }

        for (p, n) in P.iter().zip(num.iter()) {
            if n > p {
                break;
            }
            if n < p {
                return 0;
            }
        }

        for (q, n) in Q.iter().zip(num.iter()) {
            if n < q {
                break;
            }
            if n > q {
                return 0;
            }
        }

        return 1;
    }

    let mut tot = 0;
    for i in 1..=n {
        if used[i - 1] {
            continue;
        }

        num.push(i);
        used[i - 1] = true;
        tot += search(num, used, P, Q);
        num.pop();
        used[i - 1] = false;
    }
    tot
}

fn main() {
    input! {
        N: usize,
        P: [usize; N],
        Q: [usize; N],
    }

    let mut num = Vec::new();
    let mut used = vec![false; N];
    let ans = search(&mut num, &mut used, &P, &Q);
    println!("{ans}")
}
