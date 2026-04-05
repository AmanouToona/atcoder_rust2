#![allow(non_snake_case)]
use ac_library::ModInt998244353 as Mint;
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [Mint; N],
        B: [Mint; M],
    }

    let mut sum_i_ai = Mint::new(0);
    for (i, &a) in (1..).zip(A.iter()) {
        sum_i_ai += a * Mint::new(i);
    }

    // prefix sum
    let mut pref_a = vec![Mint::new(0); N + 2];
    for (i, &a) in A.iter().enumerate() {
        pref_a[i + 2] = pref_a[i + 1] + a;
    }

    let mut ans = Mint::new(0);
    for (j, &b) in (1..).zip(B.iter()) {
        let mut sub_sum = Mint::new(0);

        let mut k = 1;
        while k * j <= N {
            let left = k * j;
            let right = (N + 1).min((k + 1) * j);
            sub_sum += (pref_a[right] - pref_a[left]) * Mint::new(k) * Mint::new(j);

            k += 1;
        }

        ans += b * (sum_i_ai - sub_sum);
    }

    println!("{ans}");
}
