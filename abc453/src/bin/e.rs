#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
N が大きいから全探索ではない
L, R で全探索もできない

自分の所属していないチームに対する制約は
[N - r, N - l]

これが [l, r] と重なりを持たないなら、全てを合わせて imos で N になる領域が答えの数だが

*/

fn main() {
    input! {
        N: usize,
        lr: [(usize, usize); N],
    }

    let max: usize = 2 * 10usize.pow(5);

    let mut a: Vec<i64> = vec![0; max + 2];
    let mut b: Vec<i64> = vec![0; max + 2];
    let mut ab: Vec<i64> = vec![0; max + 2];

    for &(l, r) in lr.iter() {
        a[l] += 1;
        a[r + 1] -= 1;

        b[N - r] += 1;
        b[N - l + 1] -= 1;

        let both_l = l.max(N - r);
        let both_r = r.min(N - l);
        if both_l <= both_r {
            ab[both_l] += 1;
            ab[both_r + 1] -= 1;
        }
    }
    for i in 0..=max {
        a[i + 1] += a[i];
        b[i + 1] += b[i];
        ab[i + 1] += ab[i];
    }

    let mut frac = vec![mint::new(1); max + 1];
    for i in 1..=max {
        frac[i] = frac[i - 1] * mint::new(i)
    }

    let mut ifrac = vec![mint::new(1); max + 1];
    ifrac[max] = mint::new(1) / frac[max];
    for i in (1..=max).rev() {
        ifrac[i - 1] = ifrac[i] * mint::new(i);
    }

    let comb = |n: usize, m: usize| {
        if m > n {
            return mint::new(0);
        }
        frac[n] * ifrac[m] * ifrac[n - m]
    };

    let mut ans = mint::new(0);
    for i in 1..N {
        if a[i] + b[i] - ab[i] == N as i64 && i >= (a[i] - ab[i]) as usize {
            ans += comb(ab[i] as usize, i - (a[i] - ab[i]) as usize)
        }
    }

    println!("{ans}");
}
