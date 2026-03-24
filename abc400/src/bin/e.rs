#![allow(non_snake_case)]
use num_integer::Roots;
use proconio::input;
use std::collections::BTreeSet;
use std::ops::Bound;

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }

        primes.push(i);
        let mut j = i * i;
        while j <= n {
            is_prime[j] = false;
            j += i;
        }
    }

    primes
}

fn main() {
    input! {Q: usize, A:[ usize; Q]}

    let a_max = 10usize.pow(12);
    let primes = eratosthenes(10usize.pow(6));

    // n = (p ** k * q ** l) ** 2;
    let mut candidate: BTreeSet<usize> = BTreeSet::new();
    for (i, &p) in primes.iter().enumerate() {
        let mut pk: usize = 1;
        for _ in 1..=40 {
            pk = pk.saturating_mul(p);
            if pk.saturating_mul(pk) > a_max {
                break;
            }

            for &q in primes.iter().skip(i + 1) {
                if (pk.saturating_mul(q)).saturating_mul(pk.saturating_mul(q)) > a_max {
                    break;
                }

                let mut ok = 1;
                let mut ng = ((a_max / (pk * q)).sqrt() + 2) as u32;
                while ng - ok > 1 {
                    let mid = (ng + ok) / 2;

                    if (pk.saturating_mul(q.saturating_pow(mid)))
                        .saturating_mul(pk.saturating_mul(q.saturating_pow(mid)))
                        > a_max
                    {
                        ng = mid;
                    } else {
                        ok = mid
                    }
                }
                for i in 1..=ok {
                    candidate.insert((pk * q.pow(i)) * (pk * q.pow(i)));
                }
            }
        }
    }

    for &a in A.iter() {
        let ans = candidate
            .range((Bound::Included(&1), Bound::Included(&a)))
            .next_back()
            .unwrap();
        println!("{ans}");
    }
}
