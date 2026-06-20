#![allow(non_snake_case)]
use proconio::input;

/*
使えそうな制約 idea
- 素因数分解
- N <= K
- K が 10 **6 を超える素数をふくむならば、その素数が答え.
    - 10 ** 6 を超える複数個の素数からなることは K <= 10 ** 12 よりあり得ない
- K = O(10**12) なので sqrt(K) の制約が必要
- K が 10 ** 6 以下の素数のみで構成されるならば
    - 10 ** 6 までの N! であわらすことができるのではないか？

K が10**12 付近の素数である場合は？

*/

fn main() {
    input! {K: usize}

    let mut osa = vec![0; 2_000_001];
    for i in 2..=2_000_000 {
        if osa[i] == 0 {
            osa[i] = i;
            let mut j = i * i;
            while j <= 2_000_000 {
                osa[j] = i;
                j += i;
            }
        }
    }

    let mut k = K;
    for i in 2..=2_000_000 {
        let mut j = i;
        while j >= 2 {
            if k % osa[j] == 0 {
                k /= osa[j];
            }
            j = j / osa[j];
        }
        if k == 1 {
            println!("{i}");
            return;
        }
    }
    println!("{k}");
}
