#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
fn main() {
    input! {
        N: usize,
        lr: [(usize, usize); N],
    }
    /*
    全ての可能性を考える 愚直だと 2 ** N になる。
    a, b のチームのどちらかを考えて最後に2倍すれば一般性を失わない.
    dp らしさを感じる
    l, r の制約の確認を逐次行うことが難しい。　最後にできる人数？, 最もきつい制約を持っておけばよい？
    l でソートして?
    -> a, b に入り得るものを管理すればよい。
     */

    let mut fact = vec![mint::new(1); N + 1];
    let mut ifact = vec![mint::new(1); N + 1];

    for i in 2..=N {
        fact[i] = fact[i - 1] * mint::new(i);
        ifact[i] = ifact[i - 1] / mint::new(i);
    }

    let mut pref_1 = vec![0i64; N + 2];
    let mut pref_2 = vec![0i64; N + 2];
    let mut both = vec![0i64; N + 2];
    let mut never = vec![0i64; N + 2];

    for &(l1, r1) in lr.iter() {
        let l2 = N - r1;
        let r2 = N - l1;

        pref_1[l1] += 1;
        pref_1[r1 + 1] -= 1;

        pref_2[l2] += 1;
        pref_2[r2 + 1] -= 1;

        if (l1 <= l2 && l2 <= r1) || (l2 <= l1 && l1 <= r2) {
            // 区間が重ねっている
            both[l1.max(l2)] += 1;
            both[r1.min(r2) + 1] -= 1;

            never[0] += 1;
            never[l1.min(l2)] -= 1;

            never[r1.max(r2) + 1] += 1;
            *never.last_mut().unwrap() -= 1;
        } else {
            // 区間が重なっていない
            never[0] += 1;
            *never.last_mut().unwrap() -= 1;
            never[l1] -= 1;
            never[r1 + 1] += 1;
            never[l2] -= 1;
            never[r2 + 1] += 1;
        }
    }

    for i in 0..=N {
        pref_1[i + 1] += pref_1[i];
        pref_2[i + 1] += pref_2[i];
        both[i + 1] += both[i];
        never[i + 1] += never[i];
    }

    let mut ans = mint::new(0);
    for i in 1..N {
        if never[i] > 0 {
            continue;
        }

        let only_1 = pref_1[i] - both[i];
        if only_1 > i as i64 {
            continue;
        }

        let need_from_both = i as i64 - only_1;
        if need_from_both > both[i] {
            continue;
        }
        // both C need_from_both
        let n = both[i] as usize;
        let r = need_from_both as usize;

        ans += fact[n] * ifact[r] * ifact[n - r];
    }

    println!("{ans}");

    // eprintln!("{:?}", pref_1);
    // eprintln!("{:?}", pref_2);
    // eprintln!("{:?}", both);
    // eprintln!("{:?}", never);
}
