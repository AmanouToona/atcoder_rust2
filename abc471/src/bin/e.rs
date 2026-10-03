#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

fn comb(n: usize, k: usize) -> mint {
    if k == 0 {
        return mint::new(1);
    }
    if n == 0 {
        return mint::new(0);
    }

    let mut frac = vec![mint::new(1); n + 1];
    for i in 1..=n {
        let pre = frac[i - 1];
        frac[i] *= pre * mint::new(i);
    }

    let mut ifrac = vec![mint::new(1); n + 1];
    ifrac[n] = mint::new(1) / frac[n];
    for i in (0..n).rev() {
        let pre = ifrac[i + 1];
        ifrac[i] = pre * mint::new(i + 1);
    }

    frac[n] * ifrac[k] * ifrac[n - k]
}

fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N]
    }
    if N == 1 {
        println!("{}", mint::new(A[0] * A[0]));
        return;
    }
    if K == 1 {
        let mut ans = mint::new(0);
        for &a in A.iter() {
            ans += mint::new(a * a);
        }
        println!("{ans}");
        return;
    }

    let mut ans = mint::new(0);

    // 2乗部分
    for a in A.iter() {
        ans += mint::new(a * a);
    }
    ans *= comb(N - 1, K - 1);

    // 2ab, 2ac ... の部分
    let a_sum = mint::new(A.iter().sum::<usize>());
    let mut tmp = mint::new(0);
    for &a in A.iter() {
        tmp += (a_sum - a) * a;
    }
    ans += tmp * comb(N - 2, K - 2);

    println!("{ans}");
}
