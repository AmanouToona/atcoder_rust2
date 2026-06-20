#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
2 で区切られた空間に 1, 3 を充填する

空間の数は x2 + 1 個
ここから i 個を選択して 1 で充填する
(x2 + 1)Ci 通りの選択方法がある
1 の振り分け方法, 最低でも 1 つ入らなければならないので、 i 個の振り分けは決定。残りの x1 - i 個の振り分け方法は
x1 - 1 C i - 1

残りに 3 を振り分ける
x3 + (x2 + 1 - i) - 1 C (x2 + 1 - i - 1)
*/

fn main() {
    input! {
        (x1, x2, x3): (usize, usize, usize),
    }

    let t6: usize = 10usize.pow(6);
    let mut frac = vec![mint::new(1); t6 * 3 + 1];
    for i in 1..=t6 * 3 {
        frac[i] = frac[i - 1] * mint::new(i);
    }

    let mut ifrac = vec![mint::new(1); t6 * 3 + 1];
    ifrac[t6 * 3] = mint::new(1) / frac[t6 * 3];
    for i in (1..=t6 * 3).rev() {
        ifrac[i - 1] = ifrac[i] * mint::new(i);
    }

    let com = |n: usize, m: usize| frac[n] * ifrac[m] * ifrac[n - m];

    let mut ans = mint::new(0);

    let k = x2 + 1; // 2 が区切る空間の数

    // i := 1 を充填する空間の数
    for i in 1..=x2.min(x1) {
        // j := 3 を充填する空間の数
        let j = x2 + 1 - i;

        ans += com(k, i) * com(x1 - 1, i - 1) * com(x3 + j - 1, j - 1);
    }

    println!("{ans}");
}
