#![allow(non_snake_case)]
use num_integer::Roots;
use proconio::input;

fn eratosthenes(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }

    let mut res = Vec::new();
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n {
        if is_prime[i] {
            res.push(i);
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    res
}

fn main() {
    input! {
        (L, R): (usize, usize),
    }

    let primes = eratosthenes(R.sqrt() + 1);
    let mut is = vec![true; R - L + 1];

    for &p in primes.iter() {
        let start = ((L - 1) / p + 1).max(2) * p;
        for i in (start..=R).step_by(p) {
            is[i - L] = false;
        }
    }

    for &p in primes.iter() {
        let mut i = p;
        while i <= R {
            if i >= L {
                is[i - L] = true;
            }

            match i.checked_mul(p) {
                Some(v) => i = v,
                _ => {
                    break;
                }
            }
        }
    }

    let ans: usize = is.iter().map(|x| *x as usize).sum::<usize>() + if is[0] { 0 } else { 1 };
    println!("{ans}");
}
