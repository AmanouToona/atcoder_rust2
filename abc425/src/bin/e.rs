#![allow(non_snake_case)]
use proconio::input;

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    if n < 2 {
        return primes;
    }
    let mut tf = vec![true; n + 1];
    tf[0] = false;
    tf[1] = false;

    for i in 0..=n {
        if !tf[i] {
            continue;
        }
        primes.push(i);
        let mut j = i + i;
        while j <= n {
            tf[j] = false;
            j += i;
        }
    }

    primes
}

fn main() {
    input! {
        (T, M) : (usize, usize),
    }

    let max = 5000;
    let primes = eratosthenes(max);
    let mut count = vec![vec![0; primes.len()]; max + 1]; // n! に出現する素数 i の個数 count[n][i];

    for (i, &p) in primes.iter().enumerate() {
        let mut j = p;
        while j <= max {
            let mut k = j;
            while k <= max {
                count[k][i] += 1;
                k += j;
            }
            j *= p;
        }
    }

    for n in 1..=max {
        for i in 0..primes.len() {
            count[n][i] += count[n - 1][i];
        }
    }

    for _ in 0..T {
        input! {
            N: usize,
            C: [usize; N],
        }

        let mut sum = count[C.iter().sum::<usize>()].clone();

        for &c in C.iter() {
            for (i, p) in count[c].iter().enumerate() {
                sum[i] -= *p;
            }
        }
        let mut ans = 1;
        for (&p, &n) in primes.iter().zip(sum.iter()) {
            for _ in 0..n {
                ans *= p;
                ans %= M;
            }
        }
        println!("{ans}");
    }
}
