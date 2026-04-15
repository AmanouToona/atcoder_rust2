#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
/*
N! が K の倍数 ... K を素因数分解して 素因数のがすが会うまで N を増やす
K が 10 ** 12 まであり得て非常に大きい sqrt の計算量にならないと
eratosthenes で 10**6 までの素数を列挙して試し割すれば大丈夫

次は N だが... 少なくともKに含まれる素因数の最大値よりも大きい値になる。
N! までの間に存在する素数を効率よく列挙できるだろうか？
10 ** 6 までの素因数の個数は osa でかなり高速に列挙できそう。
10 ** 6 を超える素因数は K に同一のものは複数回は含まれない。

10 ** 6 を超えない素因数の個数が足りるか？　を計算する必要がある。
そもそも、そのものが含まれるのだから足りることは確定できる。

osa まで行う必要があるだろうか？
最大いくつまで考えればいいのだろうか？
*/

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut is_primse = vec![true; n + 1];
    is_primse[0] = false;
    is_primse[1] = false;

    for i in 2..=n {
        if !is_primse[i] {
            continue;
        }

        res.push(i);
        let mut j = i * i;
        while j <= n {
            is_primse[j] = false;
            j += i;
        }
    }

    res
}

fn main() {
    input! {
        K: usize
    }

    let t6 = 10usize.pow(6);
    let t8 = 10usize.pow(8);

    let mut k = K;
    let primse = eratosthenes(t6);
    let mut k_primes: HashMap<usize, usize> = HashMap::new();
    for p in primse.iter().rev() {
        while k % p == 0 {
            *k_primes.entry(*p).or_default() += 1;
            k /= p
        }
    }

    if k >= t6 {
        println!("{k}");
        return;
    }

    let mut osa = vec![0; t8 + 1];
    osa[0] = 1;
    osa[1] = 1;

    for i in 2..=t8 {
        if osa[i] != 0 {
            continue;
        }
        osa[i] = i;
        let mut j = i * i;
        while j <= t8 {
            osa[j] = i;
            j += i;
        }
    }

    for i in 2..=t8 {
        let mut j = i;
        while j != 1 {
            if k_primes.contains_key(&osa[j]) {
                *k_primes.entry(osa[j]).or_default() -= 1;
                if k_primes[&osa[j]] == 0 {
                    k_primes.remove(&osa[j]);
                }
            }
            j /= osa[j];
        }

        if k_primes.is_empty() {
            println!("{i}");
            return;
        }
    }
}
