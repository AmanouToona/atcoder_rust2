use ac_library::ModInt998244353 as mint;
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn eratosthenes(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }

    let mut res = Vec::new();
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }
        res.push(i);

        let mut j = i;
        while j + i <= n {
            is_prime[j + i] = false;
            j += i;
        }
    }

    res
}

#[allow(non_snake_case)]
fn main() {
    input! {T: usize}
    let primes = eratosthenes(10_000_000);
    for _ in 0..T {
        input! {
            N: usize,
            A: [usize; N],
        }

        let mut primes_cnt: HashMap<usize, (u64, u64)> = HashMap::new();
        let mut a_primes = vec![Vec::new(); N];
        for (i, &a) in A.iter().enumerate() {
            let mut muta = a;

            for &p in primes.iter() {
                let mut cnt = 0;
                while muta >= p && muta % p == 0 {
                    cnt += 1;
                    muta /= p;
                }

                if cnt != 0 {
                    a_primes[i].push((p, cnt));
                    let u = primes_cnt.entry(p).or_default();

                    let mut v = *u;
                    if u.0 <= cnt {
                        v.0 = cnt;
                        v.1 = u.0;
                    } else if v.1 < cnt {
                        v.1 = cnt;
                    }

                    u.0 = v.0;
                    u.1 = v.1;
                }
            }
        }

        let mut all = mint::new(1);
        for (k, v) in primes_cnt.iter() {
            all *= mint::new(*k).pow(v.0);
        }
        let mut ans = Vec::new();
        for (i, a) in a_primes.iter().enumerate() {
            let mut sub = mint::new(1);
            for &(v, c) in a.iter() {
                let &(v0, v1) = primes_cnt.get(&v).unwrap();
                if v0 == c {
                    sub *= mint::new(v).pow(c - v1);
                }
            }
            ans.push(all / sub);
        }
        let ans: String = ans.iter().join(" ");
        println!("{ans}");
    }
}
