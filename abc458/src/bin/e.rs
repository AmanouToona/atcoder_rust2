#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
1 と 3 が隣接してはいけない

x := x1 + x2 + x3

制約なしでは、Aの個数は
xCx1 * (x - x1)Cx2 個

ここから、 1, 3 が隣接する A を排除できれば解になる

「1の両隣は必ず 1 or 2 である」という言い換えもできる
「3の両隣は必ず 3 or 2 である」という言い換えもできる

1 の隣接の個数について全探索するのはどうだろうか？
xx >= 1 より、 1の隣接の個数は 1..=2*x1 (上限は条件で変動)

2 で空間 s を 2..=x2 + 1　個に区切る
そこに 1, or 3 を充填する
充填方法は何通り?
空間を 1, 3 のどちらに当てるか？で、 2 ** s - 2 通り
1 を各空間に何個入れるか？ が何通り？ 計算量が多すぎる

1 から決めていく？
1 を x 個に分ける
隣には2 を配置する
3 を適当に入れる
なんかこっちの気がするけど解く時間がない

*/

fn main() {
    input! {
        (x1, x2, x3): (usize, usize, usize),
    }

    let t6 = 1_000_000;
    let mut frac = vec![mint::new(1); t6 + 1];
    for i in 2..=t6 {
        frac[i] = frac[i - 1] * mint::new(i);
    }

    let mut ifrac = vec![mint::new(1); t6 + 1];
    ifrac[t6] = mint::new(1) / frac[t6];
    for i in (2..=t6).rev() {
        ifrac[i - 1] = ifrac[i] * mint::new(i);
    }

    let mut tpow = vec![mint::new(1); t6 + 3];
    for i in 1..tpow.len() {
        tpow[i] = tpow[i - 1] * mint::new(2);
    }

    let mut ans = mint::new(0);
    for gap in 2..=x2 + 1 {
        // gap の発生箇所の組み合わせはいくつある? (x2 + 1) C gap
        let spot = frac[x2 + 1] * ifrac[gap] * ifrac[x2 + 1 - gap];

        // gap を 1, 3 のどちらで埋めるか  2 ** gap - 2 通り (どちらか一方のみはなしなので -2した)
        let witch = tpow[gap] - 2;

        // 1 の分け方
        let div1 = mint::new(gap).pow(x1 as u64);

        // 3 の分け方
        let div3 = mint::new(gap).pow(x3 as u64);

        eprintln!("{gap} {spot} {witch} {div1} {div3} ");
        ans += spot * witch;
    }

    println!("{ans}");
}
